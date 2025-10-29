import { vi } from 'vitest'

export const getStores = () => {
	const navigating = { subscribe: vi.fn() }
	const page = { subscribe: vi.fn() }
	const updated = { subscribe: vi.fn(), check: vi.fn() }
	return { navigating, page, updated }
}

export const page = { subscribe: vi.fn() }
export const navigating = { subscribe: vi.fn() }
export const updated = { subscribe: vi.fn(), check: vi.fn() }
