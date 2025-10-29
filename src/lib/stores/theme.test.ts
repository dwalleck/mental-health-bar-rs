import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest'
import { get } from 'svelte/store'

// Mock $app/environment to return browser: true
vi.mock('$app/environment', () => ({
	browser: true,
}))

describe('Theme Store', () => {
	let mockLocalStorage: { [key: string]: string }
	let mockClassList: { add: ReturnType<typeof vi.fn>; remove: ReturnType<typeof vi.fn> }
	let mediaQueryListeners: ((event: MediaQueryListEvent) => void)[]

	beforeEach(() => {
		// Reset module state
		vi.resetModules()

		// Re-mock $app/environment after reset
		vi.doMock('$app/environment', () => ({
			browser: true,
		}))

		// Mock localStorage
		mockLocalStorage = {}
		global.localStorage = {
			getItem: vi.fn((key: string) => mockLocalStorage[key] || null),
			setItem: vi.fn((key: string, value: string) => {
				mockLocalStorage[key] = value
			}),
			removeItem: vi.fn((key: string) => {
				delete mockLocalStorage[key]
			}),
			clear: vi.fn(() => {
				mockLocalStorage = {}
			}),
			key: vi.fn(() => null),
			length: 0,
		}

		// Mock document.documentElement.classList
		mockClassList = {
			add: vi.fn(),
			remove: vi.fn(),
		}
		Object.defineProperty(global.document, 'documentElement', {
			value: {
				classList: mockClassList,
			},
			writable: true,
			configurable: true,
		})

		// Mock window.matchMedia
		mediaQueryListeners = []
		global.window = {
			...global.window,
			matchMedia: vi.fn((query: string) => ({
				matches: false,
				media: query,
				onchange: null,
				addListener: vi.fn(),
				removeListener: vi.fn(),
				addEventListener: vi.fn((event: string, listener: (event: MediaQueryListEvent) => void) => {
					if (event === 'change') {
						mediaQueryListeners.push(listener)
					}
				}),
				removeEventListener: vi.fn(),
				dispatchEvent: vi.fn(),
			})),
		}
	})

	afterEach(() => {
		vi.restoreAllMocks()
	})

	describe('Initial State', () => {
		it('should default to system when no stored theme', async () => {
			const { theme } = await import('./theme')
			expect(get(theme)).toBe('system')
		})

		it('should load light theme from localStorage', async () => {
			mockLocalStorage['theme'] = 'light'
			const { theme } = await import('./theme')
			expect(get(theme)).toBe('light')
		})

		it('should load dark theme from localStorage', async () => {
			mockLocalStorage['theme'] = 'dark'
			const { theme } = await import('./theme')
			expect(get(theme)).toBe('dark')
		})

		it('should load system theme from localStorage', async () => {
			mockLocalStorage['theme'] = 'system'
			const { theme } = await import('./theme')
			expect(get(theme)).toBe('system')
		})

		it('should ignore invalid stored theme', async () => {
			mockLocalStorage['theme'] = 'invalid'
			const { theme } = await import('./theme')
			expect(get(theme)).toBe('system')
		})
	})

	describe('Theme Updates', () => {
		it('should update to dark theme', async () => {
			const { theme } = await import('./theme')
			theme.set('dark')
			expect(get(theme)).toBe('dark')
		})

		it('should update to light theme', async () => {
			const { theme } = await import('./theme')
			theme.set('light')
			expect(get(theme)).toBe('light')
		})

		it('should update to system theme', async () => {
			const { theme } = await import('./theme')
			theme.set('system')
			expect(get(theme)).toBe('system')
		})
	})

	describe('localStorage Persistence', () => {
		it('should save theme to localStorage on change', async () => {
			const { theme } = await import('./theme')
			theme.set('dark')

			// Wait for subscription to run
			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(localStorage.setItem).toHaveBeenCalledWith('theme', 'dark')
		})

		it('should persist light theme', async () => {
			const { theme } = await import('./theme')
			theme.set('light')

			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(mockLocalStorage['theme']).toBe('light')
		})

		it('should persist system theme', async () => {
			const { theme } = await import('./theme')
			theme.set('system')

			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(mockLocalStorage['theme']).toBe('system')
		})
	})

	describe('DOM Updates', () => {
		it('should add dark class when set to dark', async () => {
			const { theme } = await import('./theme')
			theme.set('dark')

			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(mockClassList.add).toHaveBeenCalledWith('dark')
		})

		it('should remove dark class when set to light', async () => {
			const { theme } = await import('./theme')
			theme.set('light')

			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(mockClassList.remove).toHaveBeenCalledWith('dark')
		})

		it('should remove dark class for system theme when system prefers light', async () => {
			;(window.matchMedia as ReturnType<typeof vi.fn>).mockReturnValue({
				matches: false,
				media: '(prefers-color-scheme: dark)',
				addEventListener: vi.fn(),
			})

			const { theme } = await import('./theme')
			theme.set('system')

			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(mockClassList.remove).toHaveBeenCalledWith('dark')
		})

		it('should add dark class for system theme when system prefers dark', async () => {
			;(window.matchMedia as ReturnType<typeof vi.fn>).mockReturnValue({
				matches: true,
				media: '(prefers-color-scheme: dark)',
				addEventListener: vi.fn(),
			})

			vi.resetModules()
			const { theme } = await import('./theme')
			theme.set('system')

			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(mockClassList.add).toHaveBeenCalledWith('dark')
		})
	})

	describe('Theme Subscription', () => {
		it('should allow subscribing to theme changes', async () => {
			const { theme } = await import('./theme')
			const values: string[] = []

			const unsubscribe = theme.subscribe((value) => {
				values.push(value)
			})

			theme.set('dark')
			theme.set('light')

			unsubscribe()

			expect(values).toContain('dark')
			expect(values).toContain('light')
		})

		it('should notify subscribers on update', async () => {
			const { theme } = await import('./theme')
			const callback = vi.fn()

			theme.subscribe(callback)

			theme.set('dark')

			expect(callback).toHaveBeenCalledWith('dark')
		})
	})

	describe('System Theme Changes', () => {
		it('should register listener for system theme changes', async () => {
			await import('./theme')

			expect(window.matchMedia).toHaveBeenCalledWith('(prefers-color-scheme: dark)')
		})

		it('should update DOM when system theme changes to dark', async () => {
			const { theme } = await import('./theme')
			theme.set('system')

			await new Promise((resolve) => setTimeout(resolve, 0))
			mockClassList.add.mockClear()
			mockClassList.remove.mockClear()

			// Simulate system theme change to dark
			if (mediaQueryListeners.length > 0) {
				mediaQueryListeners[0]({ matches: true } as MediaQueryListEvent)
			}

			// DOM should update to dark mode
			expect(mockClassList.add).toHaveBeenCalledWith('dark')
			expect(get(theme)).toBe('system')
		})

		it('should update DOM when system theme changes to light', async () => {
			// Start with dark system theme
			;(window.matchMedia as ReturnType<typeof vi.fn>).mockReturnValue({
				matches: true,
				media: '(prefers-color-scheme: dark)',
				addEventListener: vi.fn((event: string, listener: (event: MediaQueryListEvent) => void) => {
					if (event === 'change') {
						mediaQueryListeners.push(listener)
					}
				}),
			})

			vi.resetModules()
			const { theme } = await import('./theme')
			theme.set('system')

			await new Promise((resolve) => setTimeout(resolve, 0))
			mockClassList.add.mockClear()
			mockClassList.remove.mockClear()

			// Simulate system theme change to light
			if (mediaQueryListeners.length > 0) {
				mediaQueryListeners[0]({ matches: false } as MediaQueryListEvent)
			}

			// DOM should update to light mode
			expect(mockClassList.remove).toHaveBeenCalledWith('dark')
			expect(get(theme)).toBe('system')
		})

		it('should not change DOM when theme is explicitly set and system changes', async () => {
			const { theme } = await import('./theme')
			theme.set('light')

			await new Promise((resolve) => setTimeout(resolve, 0))
			mockClassList.add.mockClear()
			mockClassList.remove.mockClear()

			// Simulate system theme change
			if (mediaQueryListeners.length > 0) {
				mediaQueryListeners[0]({ matches: true } as MediaQueryListEvent)
			}

			// DOM should NOT update (user has explicit preference)
			expect(mockClassList.add).not.toHaveBeenCalled()
			expect(mockClassList.remove).not.toHaveBeenCalled()
			expect(get(theme)).toBe('light')
		})
	})

	describe('Type Safety', () => {
		it('should accept valid theme values', async () => {
			const { theme } = await import('./theme')

			theme.set('light')
			expect(get(theme)).toBe('light')

			theme.set('dark')
			expect(get(theme)).toBe('dark')

			theme.set('system')
			expect(get(theme)).toBe('system')
		})
	})

	describe('Integration Scenarios', () => {
		it('should handle full theme cycle', async () => {
			const { theme } = await import('./theme')

			// Start with light
			theme.set('light')
			await new Promise((resolve) => setTimeout(resolve, 0))
			expect(mockLocalStorage['theme']).toBe('light')
			expect(mockClassList.remove).toHaveBeenCalledWith('dark')

			// Switch to dark
			theme.set('dark')
			await new Promise((resolve) => setTimeout(resolve, 0))
			expect(mockLocalStorage['theme']).toBe('dark')
			expect(mockClassList.add).toHaveBeenCalledWith('dark')

			// Switch to system
			theme.set('system')
			await new Promise((resolve) => setTimeout(resolve, 0))
			expect(mockLocalStorage['theme']).toBe('system')
		})

		it('should persist across multiple updates', async () => {
			const { theme } = await import('./theme')

			theme.set('light')
			await new Promise((resolve) => setTimeout(resolve, 0))

			theme.set('dark')
			await new Promise((resolve) => setTimeout(resolve, 0))

			theme.set('light')
			await new Promise((resolve) => setTimeout(resolve, 0))

			expect(mockLocalStorage['theme']).toBe('light')
		})
	})
})
