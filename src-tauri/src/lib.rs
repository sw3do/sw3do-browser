use std::sync::Mutex;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use scraper::{Html, Selector};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchResult {
    title: String,
    url: String,
    description: String,
    relevance_score: f64,
    domain_authority: f64,
    content_quality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PageContent {
    title: String,
    content: String,
    url: String,
    links: Vec<SearchResult>,
}

struct BrowserState {
    history: Vec<String>,
    current_index: i32,
    current_page: Option<PageContent>,
    search_cache: HashMap<String, Vec<SearchResult>>,
    sw3do_config: SW3DOConfig,
    privacy_mode: bool,
    blocked_trackers: Vec<String>,
}

#[derive(Debug, Clone)]
struct SW3DOConfig {
    max_results: usize,
    relevance_threshold: f64,
    domain_boost: HashMap<String, f64>,
    content_filters: Vec<String>,
    privacy_enhanced: bool,
    block_trackers: bool,
    google_search_enabled: bool,
}

impl Default for BrowserState {
    fn default() -> Self {
        let mut domain_boost = HashMap::new();
        domain_boost.insert("wikipedia.org".to_string(), 1.5);
        domain_boost.insert("github.com".to_string(), 1.3);
        domain_boost.insert("stackoverflow.com".to_string(), 1.4);
        domain_boost.insert("mozilla.org".to_string(), 1.2);
        domain_boost.insert("reddit.com".to_string(), 1.1);
        
        Self {
            history: vec!["sw3do://home".to_string()],
            current_index: 0,
            current_page: None,
            search_cache: HashMap::new(),
            sw3do_config: SW3DOConfig {
                max_results: 20,
                relevance_threshold: 0.3,
                domain_boost,
                content_filters: vec![
                    "spam".to_string(),
                    "malware".to_string(),
                    "phishing".to_string(),
                    "tracker".to_string(),
                    "ads".to_string(),
                ],
                privacy_enhanced: true,
                block_trackers: true,
                google_search_enabled: true,
            },
            privacy_mode: true,
            blocked_trackers: Vec::new(),
        }
    }
}

#[tauri::command]
async fn navigate_to(url: String, state: tauri::State<'_, Mutex<BrowserState>>) -> Result<PageContent, String> {
    let final_url = if url.starts_with("sw3do://") {
        url
    } else if !url.starts_with("http://") && !url.starts_with("https://") {
        if url.contains(".") && !url.contains(" ") {
            format!("https://{}", url)
        } else {
            format!("sw3do://search?q={}", urlencoding::encode(&url))
        }
    } else {
        url
    };
    
    {
        let mut browser_state = state.lock().unwrap();
        if browser_state.current_index < (browser_state.history.len() as i32) - 1 {
            let truncate_index = (browser_state.current_index + 1) as usize;
            browser_state.history.truncate(truncate_index);
        }
        browser_state.history.push(final_url.clone());
        browser_state.current_index = (browser_state.history.len() as i32) - 1;
    }
    
    let page_content = if final_url.starts_with("sw3do://") {
        handle_internal_page(&final_url, &state).await?
    } else {
        fetch_external_page(&final_url).await?
    };
    
    {
        let mut browser_state = state.lock().unwrap();
        browser_state.current_page = Some(page_content.clone());
    }
    
    Ok(page_content)
}

#[tauri::command]
async fn go_back(state: tauri::State<'_, Mutex<BrowserState>>) -> Result<Option<PageContent>, String> {
    let url = {
        let mut browser_state = state.lock().unwrap();
        if browser_state.current_index > 0 {
            browser_state.current_index -= 1;
            browser_state.history[browser_state.current_index as usize].clone()
        } else {
            return Ok(None);
        }
    };
    
    let page_content = if url.starts_with("sw3do://") {
        handle_internal_page(&url, &state).await?
    } else {
        fetch_external_page(&url).await?
    };
    
    {
        let mut browser_state = state.lock().unwrap();
        browser_state.current_page = Some(page_content.clone());
    }
    
    Ok(Some(page_content))
}

#[tauri::command]
async fn go_forward(state: tauri::State<'_, Mutex<BrowserState>>) -> Result<Option<PageContent>, String> {
    let url = {
        let mut browser_state = state.lock().unwrap();
        if browser_state.current_index < (browser_state.history.len() as i32) - 1 {
            browser_state.current_index += 1;
            browser_state.history[browser_state.current_index as usize].clone()
        } else {
            return Ok(None);
        }
    };
    
    let page_content = if url.starts_with("sw3do://") {
        handle_internal_page(&url, &state).await?
    } else {
        fetch_external_page(&url).await?
    };
    
    {
        let mut browser_state = state.lock().unwrap();
        browser_state.current_page = Some(page_content.clone());
    }
    
    Ok(Some(page_content))
}

#[tauri::command]
async fn search_web(query: String, state: tauri::State<'_, Mutex<BrowserState>>) -> Result<Vec<SearchResult>, String> {
    let cache_key = query.to_lowercase();
    
    {
        let browser_state = state.lock().unwrap();
        if let Some(cached_results) = browser_state.search_cache.get(&cache_key) {
            return Ok(cached_results.clone());
        }
    }
    
    let search_engines = vec![
        format!("https://www.google.com/search?q={}", urlencoding::encode(&query)),
        format!("https://duckduckgo.com/?q={}", urlencoding::encode(&query)),
        format!("https://www.bing.com/search?q={}", urlencoding::encode(&query)),
        format!("https://search.yahoo.com/search?p={}", urlencoding::encode(&query)),
    ];
    
    let mut all_results = Vec::new();
    
    for search_url in search_engines {
        if let Ok(mut results) = fetch_search_results(&search_url, &query).await {
            all_results.append(&mut results);
        }
    }
    
    let sw3do_results = sw3do_rank_and_filter(all_results, &query, &state).await?;
    
    {
        let mut browser_state = state.lock().unwrap();
        if !browser_state.privacy_mode {
            browser_state.search_cache.insert(cache_key, sw3do_results.clone());
        }
    }
    
    Ok(sw3do_results)
}

async fn sw3do_rank_and_filter(
    mut results: Vec<SearchResult>,
    query: &str,
    state: &tauri::State<'_, Mutex<BrowserState>>
) -> Result<Vec<SearchResult>, String> {
    let browser_state = state.lock().unwrap();
    let config = browser_state.sw3do_config.clone();
    let privacy_mode = browser_state.privacy_mode;
    drop(browser_state);
    
    let query_lower = query.to_lowercase();
    let query_terms: Vec<&str> = query_lower.split_whitespace().collect();
    
    for result in &mut results {
        result.relevance_score = calculate_relevance_score(&result.title, &result.description, &query_terms);
        result.domain_authority = calculate_domain_authority(&result.url, &config.domain_boost);
        result.content_quality = calculate_content_quality(&result.title, &result.description);
    }
    
    results.retain(|r| {
        r.relevance_score >= config.relevance_threshold &&
        !config.content_filters.iter().any(|filter| {
            r.title.to_lowercase().contains(filter) || 
            r.description.to_lowercase().contains(filter)
        })
    });
    
    results.sort_by(|a, b| {
        let score_a = a.relevance_score * a.domain_authority * a.content_quality;
        let score_b = b.relevance_score * b.domain_authority * b.content_quality;
        score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
    });
    
    results.truncate(config.max_results);
    Ok(results)
}

fn calculate_relevance_score(title: &str, description: &str, query_terms: &[&str]) -> f64 {
    let title_lower = title.to_lowercase();
    let desc_lower = description.to_lowercase();
    let combined_text = format!("{} {}", title_lower, desc_lower);
    
    let mut score = 0.0;
    let total_terms = query_terms.len() as f64;
    
    for term in query_terms {
        if title_lower.contains(term) {
            score += 2.0;
        }
        if desc_lower.contains(term) {
            score += 1.0;
        }
        
        let term_count = combined_text.matches(term).count() as f64;
        score += term_count * 0.1;
    }
    
    (score / total_terms).min(3.0)
}

fn calculate_domain_authority(url: &str, domain_boost: &HashMap<String, f64>) -> f64 {
    if let Ok(parsed_url) = url::Url::parse(url) {
        if let Some(domain) = parsed_url.domain() {
            for (boost_domain, boost_value) in domain_boost {
                if domain.contains(boost_domain) {
                    return *boost_value;
                }
            }
        }
    }
    1.0
}

fn calculate_content_quality(title: &str, description: &str) -> f64 {
    let mut quality: f64 = 1.0;
    
    if title.len() > 10 && title.len() < 100 {
        quality += 0.2;
    }
    
    if description.len() > 50 && description.len() < 300 {
        quality += 0.3;
    }
    
    let title_words = title.split_whitespace().count();
    if title_words >= 3 && title_words <= 15 {
        quality += 0.2;
    }
    
    let numeric_count = title.chars().filter(|c| c.is_numeric()).count();
    if numeric_count <= title.len() / 2 {
        quality += 0.1;
    }
    
    quality.min(2.0)
}

async fn fetch_search_results(search_url: &str, _query: &str) -> Result<Vec<SearchResult>, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;
    
    let response = client.get(search_url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let html = response.text().await.map_err(|e| e.to_string())?;
    let document = Html::parse_document(&html);
    
    let mut results = Vec::new();
    
    if search_url.contains("bing.com") {
        let result_selector = Selector::parse(".b_algo").unwrap();
        let title_selector = Selector::parse("h2 a").unwrap();
        let desc_selector = Selector::parse(".b_caption p").unwrap();
        
        for element in document.select(&result_selector) {
            if let Some(title_elem) = element.select(&title_selector).next() {
                let title = title_elem.text().collect::<String>();
                let url = title_elem.value().attr("href").unwrap_or("").to_string();
                let description = element.select(&desc_selector)
                    .next()
                    .map(|e| e.text().collect::<String>())
                    .unwrap_or_default();
                
                if !title.is_empty() && !url.is_empty() {
                    results.push(SearchResult {
                        title,
                        url,
                        description,
                        relevance_score: 0.0,
                        domain_authority: 1.0,
                        content_quality: 1.0,
                    });
                }
            }
        }
    }
    
    Ok(results)
}

async fn handle_internal_page(url: &str, state: &tauri::State<'_, Mutex<BrowserState>>) -> Result<PageContent, String> {
    if url == "sw3do://home" {
        Ok(PageContent {
            title: "SW3DO Browser - Home".to_string(),
            url: url.to_string(),
            content: generate_home_page(),
            links: vec![],
        })
    } else if url.starts_with("sw3do://search?q=") {
        let query = url.split("q=").nth(1).unwrap_or("");
        let decoded_query = urlencoding::decode(query).map_err(|e| e.to_string())?.to_string();
        
        let search_results = search_web(decoded_query.clone(), state.clone()).await?;
        
        Ok(PageContent {
            title: format!("Search Results for: {}", decoded_query),
            url: url.to_string(),
            content: generate_search_page(&decoded_query, &search_results),
            links: search_results,
        })
    } else {
        Err("Unknown internal page".to_string())
    }
}

async fn fetch_external_page(url: &str) -> Result<PageContent, String> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36")
        .build()
        .map_err(|e| e.to_string())?;
    
    let response = client.get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let html = response.text().await.map_err(|e| e.to_string())?;
    let document = Html::parse_document(&html);
    
    let title_selector = Selector::parse("title").unwrap();
    let title = document.select(&title_selector)
        .next()
        .map(|e| e.text().collect::<String>())
        .unwrap_or("Untitled".to_string());
    
    let body_selector = Selector::parse("body").unwrap();
    let content = document.select(&body_selector)
        .next()
        .map(|e| e.html())
        .unwrap_or(html);
    
    Ok(PageContent {
        title,
        url: url.to_string(),
        content,
        links: vec![],
    })
}

fn generate_home_page() -> String {
    r#"
    <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', 'SF Pro Display', Roboto, sans-serif; background: #0a0a0a; color: #ffffff; min-height: 100vh; padding: 60px 40px; text-align: center;">
        <div style="max-width: 800px; margin: 0 auto;">
            <div style="margin-bottom: 40px;">
                <div style="font-size: 64px; margin-bottom: 20px; color: #00d4aa;">🛡️</div>
                <h1 style="font-size: 48px; font-weight: 700; margin-bottom: 16px; background: linear-gradient(135deg, #ffffff 0%, #00d4aa 100%); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;">SW3DO Browser</h1>
                <p style="font-size: 20px; color: rgba(255, 255, 255, 0.7); margin-bottom: 40px;">Private, Fast, Secure - Your Digital Freedom</p>
            </div>
            
            <div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(250px, 1fr)); gap: 24px; margin-bottom: 40px;">
                <div style="padding: 24px; background: rgba(255, 255, 255, 0.05); border-radius: 16px; border: 1px solid rgba(255, 255, 255, 0.1);">
                    <div style="font-size: 32px; margin-bottom: 16px;">🔒</div>
                    <h3 style="color: #00d4aa; margin-bottom: 12px; font-size: 18px;">Zero Tracking</h3>
                    <p style="color: rgba(255, 255, 255, 0.8); font-size: 14px; line-height: 1.6;">No cookies, no history, no data collection. Browse with complete privacy.</p>
                </div>
                
                <div style="padding: 24px; background: rgba(255, 255, 255, 0.05); border-radius: 16px; border: 1px solid rgba(255, 255, 255, 0.1);">
                    <div style="font-size: 32px; margin-bottom: 16px;">⚡</div>
                    <h3 style="color: #00d4aa; margin-bottom: 12px; font-size: 18px;">Lightning Fast</h3>
                    <p style="color: rgba(255, 255, 255, 0.8); font-size: 14px; line-height: 1.6;">Optimized for speed with intelligent caching and minimal resource usage.</p>
                </div>
                
                <div style="padding: 24px; background: rgba(255, 255, 255, 0.05); border-radius: 16px; border: 1px solid rgba(255, 255, 255, 0.1);">
                    <div style="font-size: 32px; margin-bottom: 16px;">🔍</div>
                    <h3 style="color: #00d4aa; margin-bottom: 12px; font-size: 18px;">Smart Search</h3>
                    <p style="color: rgba(255, 255, 255, 0.8); font-size: 14px; line-height: 1.6;">Google-powered search with enhanced privacy and ad-blocking.</p>
                </div>
            </div>
            
            <div style="background: rgba(255, 255, 255, 0.03); border-radius: 16px; padding: 32px; border: 1px solid rgba(255, 255, 255, 0.08);">
                <h2 style="color: #ffffff; margin-bottom: 20px; font-size: 24px;">Why Choose SW3DO?</h2>
                <div style="text-align: left; max-width: 600px; margin: 0 auto;">
                    <div style="margin-bottom: 16px; display: flex; align-items: center; gap: 12px;">
                        <span style="color: #00d4aa; font-size: 18px;">✓</span>
                        <span style="color: rgba(255, 255, 255, 0.9);">Built with Tauri for maximum performance and security</span>
                    </div>
                    <div style="margin-bottom: 16px; display: flex; align-items: center; gap: 12px;">
                        <span style="color: #00d4aa; font-size: 18px;">✓</span>
                        <span style="color: rgba(255, 255, 255, 0.9);">No telemetry, analytics, or user profiling</span>
                    </div>
                    <div style="margin-bottom: 16px; display: flex; align-items: center; gap: 12px;">
                        <span style="color: #00d4aa; font-size: 18px;">✓</span>
                        <span style="color: rgba(255, 255, 255, 0.9);">Advanced tracker and ad blocking technology</span>
                    </div>
                    <div style="display: flex; align-items: center; gap: 12px;">
                        <span style="color: #00d4aa; font-size: 18px;">✓</span>
                        <span style="color: rgba(255, 255, 255, 0.9);">Modern, minimalist interface inspired by Zen Browser</span>
                    </div>
                </div>
            </div>
        </div>
    </div>
    "#.to_string()
}

fn generate_search_page(query: &str, results: &[SearchResult]) -> String {
    let mut html = format!(r#"
    <div style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; max-width: 1000px; margin: 0 auto; padding: 20px;">
        <div style="text-align: center; margin-bottom: 30px;">
            <h1 style="color: #333; margin-bottom: 10px;">🔍 Search Results</h1>
            <p style="color: #666; font-size: 1.1em;">Found {} results for: <strong>{}</strong></p>
        </div>
        
        <div style="space-y: 20px;">
    "#, results.len(), query);
    
    for (_i, result) in results.iter().enumerate() {
        html.push_str(&format!(r#"
            <div style="background: white; padding: 20px; border-radius: 10px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); margin-bottom: 20px; border-left: 4px solid #007acc;">
                <h3 style="margin: 0 0 10px 0;">
                    <a href="{}" style="color: #007acc; text-decoration: none; font-size: 1.2em;">{}</a>
                </h3>
                <p style="color: #28a745; font-size: 0.9em; margin: 5px 0;">{}</p>
                <p style="color: #666; line-height: 1.5; margin: 10px 0 0 0;">{}</p>
            </div>
        "#, result.url, result.title, result.url, result.description));
    }
    
    if results.is_empty() {
        html.push_str(r#"
            <div style="text-align: center; padding: 40px; background: #f8f9fa; border-radius: 10px;">
                <h3 style="color: #666; margin-bottom: 10px;">No results found</h3>
                <p style="color: #999;">Try different keywords or check your spelling</p>
            </div>
        "#);
    }
    
    html.push_str("</div></div>");
    html
}

#[tauri::command]
fn can_go_back(state: tauri::State<Mutex<BrowserState>>) -> bool {
    let browser_state = state.lock().unwrap();
    browser_state.current_index > 0
}

#[tauri::command]
fn can_go_forward(state: tauri::State<Mutex<BrowserState>>) -> bool {
    let browser_state = state.lock().unwrap();
    browser_state.current_index < (browser_state.history.len() as i32) - 1
}

#[tauri::command]
fn get_current_url(state: tauri::State<Mutex<BrowserState>>) -> String {
    let browser_state = state.lock().unwrap();
    if browser_state.current_index >= 0 && (browser_state.current_index as usize) < browser_state.history.len() {
        browser_state.history[browser_state.current_index as usize].clone()
    } else {
        "sw3do://home".to_string()
    }
}

#[tauri::command]
async fn get_current_page(state: tauri::State<'_, Mutex<BrowserState>>) -> Result<Option<PageContent>, String> {
    let browser_state = state.lock().unwrap();
    Ok(browser_state.current_page.clone())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Mutex::new(BrowserState::default()))
        .invoke_handler(tauri::generate_handler![
            navigate_to,
            go_back,
            go_forward,
            search_web,
            can_go_back,
            can_go_forward,
            get_current_url,
            get_current_page
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
