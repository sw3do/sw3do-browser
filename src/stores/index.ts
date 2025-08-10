import { createPinia } from 'pinia'

const pinia = createPinia()

export default pinia

export * from './browser'
export * from './settings'
export * from './bookmarks'
export * from './history'
export * from './downloads'
export * from './privacy'
export * from './plugins'
export * from './sessions'