import { expect, afterEach, vi } from 'vitest'
import { cleanup } from '@testing-library/svelte'
import '@testing-library/jest-dom/vitest'

// Mock SvelteKit modules that aren't available in test environment
vi.mock('$app/navigation', () => ({
	goto: vi.fn(),
	invalidate: vi.fn(),
	invalidateAll: vi.fn(),
	preloadData: vi.fn(),
	preloadCode: vi.fn(),
	beforeNavigate: vi.fn(),
	afterNavigate: vi.fn(),
	pushState: vi.fn(),
	replaceState: vi.fn()
}))

vi.mock('$app/environment', () => ({
	browser: false,
	dev: true,
	building: false,
	version: 'test'
}))

vi.mock('$app/stores', () => {
	const getStores = () => {
		const navigating = { subscribe: vi.fn() }
		const page = { subscribe: vi.fn() }
		const updated = { subscribe: vi.fn(), check: vi.fn() }
		return { navigating, page, updated }
	}

	const page = { subscribe: vi.fn() }
	const navigating = { subscribe: vi.fn() }
	const updated = { subscribe: vi.fn(), check: vi.fn() }

	return { getStores, navigating, page, updated }
})

// Mock Tauri API
vi.mock('@tauri-apps/api/core', () => ({
	invoke: vi.fn()
}))

// Mock Web Animations API for JSDOM
if (typeof Element.prototype.animate === 'undefined') {
	Element.prototype.animate = vi.fn(() => ({
		cancel: vi.fn(),
		finish: vi.fn(),
		pause: vi.fn(),
		play: vi.fn(),
		reverse: vi.fn(),
		updatePlaybackRate: vi.fn(),
		persist: vi.fn(),
		commitStyles: vi.fn(),
		playbackRate: 1,
		playState: 'finished',
		ready: Promise.resolve(),
		finished: Promise.resolve(),
		onfinish: null,
		oncancel: null,
		onremove: null,
		currentTime: 0,
		effect: null,
		id: '',
		pending: false,
		replaceState: 'active',
		startTime: 0,
		timeline: null,
		addEventListener: vi.fn(),
		removeEventListener: vi.fn(),
		dispatchEvent: vi.fn(),
	}))
}

// Cleanup after each test
afterEach(() => {
	cleanup()
})
