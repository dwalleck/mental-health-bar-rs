import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest'
import {
	MOOD_COLORS,
	MOOD_HOVER_COLORS,
	MOOD_LABELS,
	MOOD_HEX_COLORS,
	getMoodColor,
	getMoodLabel,
	getMoodHexColor,
} from './colors'

describe('Color Constants', () => {
	describe('MOOD_COLORS', () => {
		it('should have colors for all 7 mood ratings', () => {
			expect(MOOD_COLORS[1]).toBeDefined()
			expect(MOOD_COLORS[2]).toBeDefined()
			expect(MOOD_COLORS[3]).toBeDefined()
			expect(MOOD_COLORS[4]).toBeDefined()
			expect(MOOD_COLORS[5]).toBeDefined()
			expect(MOOD_COLORS[6]).toBeDefined()
			expect(MOOD_COLORS[7]).toBeDefined()
		})

		it('should use red for Terrible (1)', () => {
			expect(MOOD_COLORS[1]).toContain('bg-red-500')
			expect(MOOD_COLORS[1]).toContain('text-white')
		})

		it('should use dark orange for Very Bad (2)', () => {
			expect(MOOD_COLORS[2]).toContain('bg-orange-500')
			expect(MOOD_COLORS[2]).toContain('text-white')
		})

		it('should use orange for Bad (3)', () => {
			expect(MOOD_COLORS[3]).toContain('bg-orange-400')
			expect(MOOD_COLORS[3]).toContain('text-white')
		})

		it('should use yellow for Ok (4)', () => {
			expect(MOOD_COLORS[4]).toContain('bg-yellow-500')
			expect(MOOD_COLORS[4]).toContain('text-white')
		})

		it('should use lime for Good (5)', () => {
			expect(MOOD_COLORS[5]).toContain('bg-lime-500')
			expect(MOOD_COLORS[5]).toContain('text-white')
		})

		it('should use green for Very Good (6)', () => {
			expect(MOOD_COLORS[6]).toContain('bg-green-500')
			expect(MOOD_COLORS[6]).toContain('text-white')
		})

		it('should use emerald for Excellent (7)', () => {
			expect(MOOD_COLORS[7]).toContain('bg-emerald-500')
			expect(MOOD_COLORS[7]).toContain('text-white')
		})

		it('should include text-white for all mood colors', () => {
			for (let i = 1; i <= 7; i++) {
				expect(MOOD_COLORS[i]).toContain('text-white')
			}
		})
	})

	describe('MOOD_HOVER_COLORS', () => {
		it('should have hover colors for all 7 mood ratings', () => {
			expect(MOOD_HOVER_COLORS[1]).toBeDefined()
			expect(MOOD_HOVER_COLORS[2]).toBeDefined()
			expect(MOOD_HOVER_COLORS[3]).toBeDefined()
			expect(MOOD_HOVER_COLORS[4]).toBeDefined()
			expect(MOOD_HOVER_COLORS[5]).toBeDefined()
			expect(MOOD_HOVER_COLORS[6]).toBeDefined()
			expect(MOOD_HOVER_COLORS[7]).toBeDefined()
		})

		it('should use hover:bg-red-600 for rating 1', () => {
			expect(MOOD_HOVER_COLORS[1]).toBe('hover:bg-red-600')
		})

		it('should use hover:bg-orange-600 for rating 2', () => {
			expect(MOOD_HOVER_COLORS[2]).toBe('hover:bg-orange-600')
		})

		it('should use hover:bg-orange-500 for rating 3', () => {
			expect(MOOD_HOVER_COLORS[3]).toBe('hover:bg-orange-500')
		})

		it('should use hover:bg-yellow-600 for rating 4', () => {
			expect(MOOD_HOVER_COLORS[4]).toBe('hover:bg-yellow-600')
		})

		it('should use hover:bg-lime-600 for rating 5', () => {
			expect(MOOD_HOVER_COLORS[5]).toBe('hover:bg-lime-600')
		})

		it('should use hover:bg-green-600 for rating 6', () => {
			expect(MOOD_HOVER_COLORS[6]).toBe('hover:bg-green-600')
		})

		it('should use hover:bg-emerald-600 for rating 7', () => {
			expect(MOOD_HOVER_COLORS[7]).toBe('hover:bg-emerald-600')
		})

		it('should use darker shade for hover states', () => {
			for (let i = 1; i <= 7; i++) {
				expect(MOOD_HOVER_COLORS[i]).toMatch(/hover:bg-\w+-[56]00/)
			}
		})
	})

	describe('MOOD_LABELS', () => {
		it('should have labels for all 7 mood ratings', () => {
			expect(MOOD_LABELS[1]).toBeDefined()
			expect(MOOD_LABELS[2]).toBeDefined()
			expect(MOOD_LABELS[3]).toBeDefined()
			expect(MOOD_LABELS[4]).toBeDefined()
			expect(MOOD_LABELS[5]).toBeDefined()
			expect(MOOD_LABELS[6]).toBeDefined()
			expect(MOOD_LABELS[7]).toBeDefined()
		})

		it('should label rating 1 as Terrible', () => {
			expect(MOOD_LABELS[1]).toBe('Terrible')
		})

		it('should label rating 2 as Very Bad', () => {
			expect(MOOD_LABELS[2]).toBe('Very Bad')
		})

		it('should label rating 3 as Bad', () => {
			expect(MOOD_LABELS[3]).toBe('Bad')
		})

		it('should label rating 4 as Ok', () => {
			expect(MOOD_LABELS[4]).toBe('Ok')
		})

		it('should label rating 5 as Good', () => {
			expect(MOOD_LABELS[5]).toBe('Good')
		})

		it('should label rating 6 as Very Good', () => {
			expect(MOOD_LABELS[6]).toBe('Very Good')
		})

		it('should label rating 7 as Excellent', () => {
			expect(MOOD_LABELS[7]).toBe('Excellent')
		})

		it('should have string labels', () => {
			for (let i = 1; i <= 7; i++) {
				expect(typeof MOOD_LABELS[i]).toBe('string')
				expect(MOOD_LABELS[i].length).toBeGreaterThan(0)
			}
		})
	})

	describe('MOOD_HEX_COLORS', () => {
		it('should have hex colors for all 7 mood ratings', () => {
			expect(MOOD_HEX_COLORS[1]).toBeDefined()
			expect(MOOD_HEX_COLORS[2]).toBeDefined()
			expect(MOOD_HEX_COLORS[3]).toBeDefined()
			expect(MOOD_HEX_COLORS[4]).toBeDefined()
			expect(MOOD_HEX_COLORS[5]).toBeDefined()
			expect(MOOD_HEX_COLORS[6]).toBeDefined()
			expect(MOOD_HEX_COLORS[7]).toBeDefined()
		})

		it('should use red hex for rating 1', () => {
			expect(MOOD_HEX_COLORS[1]).toBe('#EF4444')
		})

		it('should use dark orange hex for rating 2', () => {
			expect(MOOD_HEX_COLORS[2]).toBe('#F97316')
		})

		it('should use orange hex for rating 3', () => {
			expect(MOOD_HEX_COLORS[3]).toBe('#FB923C')
		})

		it('should use yellow hex for rating 4', () => {
			expect(MOOD_HEX_COLORS[4]).toBe('#EAB308')
		})

		it('should use lime hex for rating 5', () => {
			expect(MOOD_HEX_COLORS[5]).toBe('#84CC16')
		})

		it('should use green hex for rating 6', () => {
			expect(MOOD_HEX_COLORS[6]).toBe('#22C55E')
		})

		it('should use emerald hex for rating 7', () => {
			expect(MOOD_HEX_COLORS[7]).toBe('#10B981')
		})

		it('should be valid hex color format', () => {
			const hexPattern = /^#[0-9A-F]{6}$/i
			for (let i = 1; i <= 7; i++) {
				expect(MOOD_HEX_COLORS[i]).toMatch(hexPattern)
			}
		})
	})
})

describe('getMoodColor', () => {
	let consoleWarnSpy: ReturnType<typeof vi.spyOn>

	beforeEach(() => {
		consoleWarnSpy = vi.spyOn(console, 'warn').mockImplementation(() => {})
	})

	afterEach(() => {
		consoleWarnSpy.mockRestore()
	})

	it('should return correct color for rating 1', () => {
		expect(getMoodColor(1)).toBe(MOOD_COLORS[1])
	})

	it('should return correct color for rating 2', () => {
		expect(getMoodColor(2)).toBe(MOOD_COLORS[2])
	})

	it('should return correct color for rating 3', () => {
		expect(getMoodColor(3)).toBe(MOOD_COLORS[3])
	})

	it('should return correct color for rating 4', () => {
		expect(getMoodColor(4)).toBe(MOOD_COLORS[4])
	})

	it('should return correct color for rating 5', () => {
		expect(getMoodColor(5)).toBe(MOOD_COLORS[5])
	})

	it('should return correct color for rating 6', () => {
		expect(getMoodColor(6)).toBe(MOOD_COLORS[6])
	})

	it('should return correct color for rating 7', () => {
		expect(getMoodColor(7)).toBe(MOOD_COLORS[7])
	})

	it('should return neutral color for rating 0', () => {
		expect(getMoodColor(0)).toBe(MOOD_COLORS[4])
	})

	it('should return neutral color for rating 8', () => {
		expect(getMoodColor(8)).toBe(MOOD_COLORS[4])
	})

	it('should return neutral color for negative rating', () => {
		expect(getMoodColor(-1)).toBe(MOOD_COLORS[4])
	})

	it('should warn when rating is out of range (too low)', () => {
		getMoodColor(0)
		expect(consoleWarnSpy).toHaveBeenCalledWith('Invalid mood rating: 0, using neutral default')
	})

	it('should warn when rating is out of range (too high)', () => {
		getMoodColor(8)
		expect(consoleWarnSpy).toHaveBeenCalledWith('Invalid mood rating: 8, using neutral default')
	})

	it('should not warn for valid ratings', () => {
		getMoodColor(1)
		getMoodColor(4)
		getMoodColor(7)
		expect(consoleWarnSpy).not.toHaveBeenCalled()
	})

	it('should handle edge case of exactly 1', () => {
		expect(getMoodColor(1)).toBe(MOOD_COLORS[1])
		expect(consoleWarnSpy).not.toHaveBeenCalled()
	})

	it('should handle edge case of exactly 7', () => {
		expect(getMoodColor(7)).toBe(MOOD_COLORS[7])
		expect(consoleWarnSpy).not.toHaveBeenCalled()
	})
})

describe('getMoodLabel', () => {
	it('should return correct label for rating 1', () => {
		expect(getMoodLabel(1)).toBe('Terrible')
	})

	it('should return correct label for rating 2', () => {
		expect(getMoodLabel(2)).toBe('Very Bad')
	})

	it('should return correct label for rating 3', () => {
		expect(getMoodLabel(3)).toBe('Bad')
	})

	it('should return correct label for rating 4', () => {
		expect(getMoodLabel(4)).toBe('Ok')
	})

	it('should return correct label for rating 5', () => {
		expect(getMoodLabel(5)).toBe('Good')
	})

	it('should return correct label for rating 6', () => {
		expect(getMoodLabel(6)).toBe('Very Good')
	})

	it('should return correct label for rating 7', () => {
		expect(getMoodLabel(7)).toBe('Excellent')
	})

	it('should return Unknown for rating 0', () => {
		expect(getMoodLabel(0)).toBe('Unknown')
	})

	it('should return Unknown for rating 8', () => {
		expect(getMoodLabel(8)).toBe('Unknown')
	})

	it('should return Unknown for negative rating', () => {
		expect(getMoodLabel(-1)).toBe('Unknown')
	})

	it('should return Unknown for undefined rating', () => {
		expect(getMoodLabel(undefined as unknown as number)).toBe('Unknown')
	})

	it('should handle all valid ratings', () => {
		for (let i = 1; i <= 7; i++) {
			expect(getMoodLabel(i)).toBe(MOOD_LABELS[i])
		}
	})
})

describe('getMoodHexColor', () => {
	it('should return correct hex color for rating 1', () => {
		expect(getMoodHexColor(1)).toBe('#EF4444')
	})

	it('should return correct hex color for rating 2', () => {
		expect(getMoodHexColor(2)).toBe('#F97316')
	})

	it('should return correct hex color for rating 3', () => {
		expect(getMoodHexColor(3)).toBe('#FB923C')
	})

	it('should return correct hex color for rating 4', () => {
		expect(getMoodHexColor(4)).toBe('#EAB308')
	})

	it('should return correct hex color for rating 5', () => {
		expect(getMoodHexColor(5)).toBe('#84CC16')
	})

	it('should return correct hex color for rating 6', () => {
		expect(getMoodHexColor(6)).toBe('#22C55E')
	})

	it('should return correct hex color for rating 7', () => {
		expect(getMoodHexColor(7)).toBe('#10B981')
	})

	it('should return default hex color for rating 0', () => {
		expect(getMoodHexColor(0)).toBe(MOOD_HEX_COLORS[3])
	})

	it('should return default hex color for rating 8', () => {
		expect(getMoodHexColor(8)).toBe(MOOD_HEX_COLORS[3])
	})

	it('should return default hex color for negative rating', () => {
		expect(getMoodHexColor(-1)).toBe(MOOD_HEX_COLORS[3])
	})

	it('should return valid hex format for all valid ratings', () => {
		const hexPattern = /^#[0-9A-F]{6}$/i
		for (let i = 1; i <= 7; i++) {
			expect(getMoodHexColor(i)).toMatch(hexPattern)
		}
	})

	it('should handle all valid ratings', () => {
		for (let i = 1; i <= 7; i++) {
			expect(getMoodHexColor(i)).toBe(MOOD_HEX_COLORS[i])
		}
	})
})

describe('Color Consistency', () => {
	it('should have matching number of entries across all constants', () => {
		const colorKeys = Object.keys(MOOD_COLORS)
		const hoverKeys = Object.keys(MOOD_HOVER_COLORS)
		const labelKeys = Object.keys(MOOD_LABELS)
		const hexKeys = Object.keys(MOOD_HEX_COLORS)

		expect(colorKeys.length).toBe(7)
		expect(hoverKeys.length).toBe(7)
		expect(labelKeys.length).toBe(7)
		expect(hexKeys.length).toBe(7)
	})

	it('should have same keys across all constants', () => {
		const keys = [1, 2, 3, 4, 5, 6, 7]

		keys.forEach((key) => {
			expect(MOOD_COLORS[key]).toBeDefined()
			expect(MOOD_HOVER_COLORS[key]).toBeDefined()
			expect(MOOD_LABELS[key]).toBeDefined()
			expect(MOOD_HEX_COLORS[key]).toBeDefined()
		})
	})

	it('should match color progression from negative to positive', () => {
		// Rating 1 should be most negative (red)
		expect(MOOD_COLORS[1]).toContain('red')
		expect(MOOD_LABELS[1]).toBe('Terrible')

		// Rating 4 should be neutral (yellow)
		expect(MOOD_COLORS[4]).toContain('yellow')
		expect(MOOD_LABELS[4]).toBe('Ok')

		// Rating 7 should be most positive (emerald green)
		expect(MOOD_COLORS[7]).toContain('emerald')
		expect(MOOD_LABELS[7]).toBe('Excellent')
	})
})
