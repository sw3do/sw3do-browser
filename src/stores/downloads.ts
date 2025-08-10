import { defineStore } from 'pinia'
import { ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'

export type DownloadStatus = 'pending' | 'downloading' | 'completed' | 'failed' | 'cancelled' | 'paused'

export interface Download {
  id: string
  url: string
  filename: string
  file_path: string
  total_bytes: number
  downloaded_bytes: number
  status: DownloadStatus
  start_time: string
  end_time?: string
  error_message?: string
  mime_type?: string
  referrer?: string
}

export interface DownloadStats {
  total_downloads: number
  completed_downloads: number
  failed_downloads: number
  total_bytes_downloaded: number
  active_downloads: number
}

export const useDownloadsStore = defineStore('downloads', () => {
  const downloads = ref<Download[]>([])
  const isLoading = ref(false)

  async function loadDownloads(): Promise<void> {
    try {
      isLoading.value = true
      const downloadList = await invoke<Download[]>('get_downloads')
      downloads.value = downloadList
    } catch (error) {
      console.error('Failed to load downloads:', error)
      throw error
    } finally {
      isLoading.value = false
    }
  }

  async function startDownload(url: string, filename?: string, referrer?: string): Promise<string> {
    try {
      const downloadId = await invoke<string>('start_download', {
        url,
        filename,
        referrer
      })
      
      await loadDownloads()
      return downloadId
    } catch (error) {
      console.error('Failed to start download:', error)
      throw error
    }
  }

  async function cancelDownload(downloadId: string): Promise<void> {
    try {
      await invoke('cancel_download', { downloadId })
      
      const download = downloads.value.find(d => d.id === downloadId)
      if (download) {
        download.status = 'cancelled'
      }
    } catch (error) {
      console.error('Failed to cancel download:', error)
      throw error
    }
  }

  async function pauseDownload(downloadId: string): Promise<void> {
    try {
      await invoke('pause_download', { downloadId })
      
      const download = downloads.value.find(d => d.id === downloadId)
      if (download) {
        download.status = 'paused'
      }
    } catch (error) {
      console.error('Failed to pause download:', error)
      throw error
    }
  }

  async function resumeDownload(downloadId: string): Promise<void> {
    try {
      await invoke('resume_download', { downloadId })
      
      const download = downloads.value.find(d => d.id === downloadId)
      if (download) {
        download.status = 'downloading'
      }
    } catch (error) {
      console.error('Failed to resume download:', error)
      throw error
    }
  }

  async function removeDownload(downloadId: string): Promise<void> {
    try {
      await invoke('remove_download', { downloadId })
      downloads.value = downloads.value.filter(d => d.id !== downloadId)
    } catch (error) {
      console.error('Failed to remove download:', error)
      throw error
    }
  }

  async function clearCompletedDownloads(): Promise<void> {
    try {
      await invoke('clear_completed_downloads')
      downloads.value = downloads.value.filter(d => d.status !== 'completed')
    } catch (error) {
      console.error('Failed to clear completed downloads:', error)
      throw error
    }
  }

  async function getActiveDownloads(): Promise<Download[]> {
    try {
      const activeDownloads = await invoke<Download[]>('get_active_downloads')
      return activeDownloads
    } catch (error) {
      console.error('Failed to get active downloads:', error)
      return []
    }
  }

  async function getDownloadStats(): Promise<DownloadStats> {
    try {
      const stats = await invoke<DownloadStats>('get_download_stats')
      return stats
    } catch (error) {
      console.error('Failed to get download stats:', error)
      return {
        total_downloads: 0,
        completed_downloads: 0,
        failed_downloads: 0,
        total_bytes_downloaded: 0,
        active_downloads: 0
      }
    }
  }

  async function setDownloadDirectory(directory: string): Promise<void> {
    try {
      await invoke('set_download_directory', { directory })
    } catch (error) {
      console.error('Failed to set download directory:', error)
      throw error
    }
  }

  async function fetchDownloadProgress(downloadId: string): Promise<number> {
    try {
      const progress = await invoke<number>('get_download_progress', { downloadId })
      return progress
    } catch (error) {
      console.error('Failed to get download progress:', error)
      return 0
    }
  }

  async function exportDownloads(): Promise<string> {
    try {
      const data = await invoke<string>('export_downloads')
      return data
    } catch (error) {
      console.error('Failed to export downloads:', error)
      throw error
    }
  }

  function getDownloadById(downloadId: string): Download | undefined {
    return downloads.value.find(download => download.id === downloadId)
  }

  function getDownloadsByStatus(status: DownloadStatus): Download[] {
    return downloads.value.filter(download => download.status === status)
  }

  function getCompletedDownloads(): Download[] {
    return downloads.value.filter(download => download.status === 'completed')
  }

  function getFailedDownloads(): Download[] {
    return downloads.value.filter(download => download.status === 'failed')
  }

  function calculateDownloadProgress(download: Download): number {
    if (download.total_bytes === 0) return 0
    return Math.round((download.downloaded_bytes / download.total_bytes) * 100)
  }

  function formatFileSize(bytes: number): string {
    const units = ['B', 'KB', 'MB', 'GB', 'TB']
    let size = bytes
    let unitIndex = 0
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024
      unitIndex++
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`
  }

  function getDownloadSpeed(download: Download): string {
    if (download.status !== 'downloading' || !download.start_time) {
      return '0 B/s'
    }
    
    const startTime = new Date(download.start_time).getTime()
    const currentTime = Date.now()
    const elapsedSeconds = (currentTime - startTime) / 1000
    
    if (elapsedSeconds === 0) return '0 B/s'
    
    const bytesPerSecond = download.downloaded_bytes / elapsedSeconds
    return `${formatFileSize(bytesPerSecond)}/s`
  }

  function getEstimatedTimeRemaining(download: Download): string {
    if (download.status !== 'downloading' || download.total_bytes === 0) {
      return 'Unknown'
    }
    
    const remainingBytes = download.total_bytes - download.downloaded_bytes
    const startTime = new Date(download.start_time).getTime()
    const currentTime = Date.now()
    const elapsedSeconds = (currentTime - startTime) / 1000
    
    if (elapsedSeconds === 0 || download.downloaded_bytes === 0) {
      return 'Unknown'
    }
    
    const bytesPerSecond = download.downloaded_bytes / elapsedSeconds
    const remainingSeconds = remainingBytes / bytesPerSecond
    
    if (remainingSeconds < 60) {
      return `${Math.round(remainingSeconds)}s`
    } else if (remainingSeconds < 3600) {
      return `${Math.round(remainingSeconds / 60)}m`
    } else {
      return `${Math.round(remainingSeconds / 3600)}h`
    }
  }

  return {
    downloads,
    isLoading,
    loadDownloads,
    startDownload,
    cancelDownload,
    pauseDownload,
    resumeDownload,
    removeDownload,
    clearCompletedDownloads,
    getActiveDownloads,
    getDownloadStats,
    setDownloadDirectory,
    fetchDownloadProgress,
    exportDownloads,
    getDownloadById,
    getDownloadsByStatus,
    getCompletedDownloads,
    getFailedDownloads,
    formatFileSize,
    calculateDownloadProgress,
    getDownloadSpeed,
    getEstimatedTimeRemaining
  }
})