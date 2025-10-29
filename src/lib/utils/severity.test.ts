import { describe, it, expect } from 'vitest'
import {
	getSeverityColor,
	getSeverityBgColor,
	getSeverityRanges,
	formatSeverity,
	calculateSegmentWidth,
} from './severity'

describe('severity utility functions', () => {
	describe('getSeverityColor', () => {
		it('should return correct color classes for minimal severity', () => {
			const color = getSeverityColor('minimal')
			expect(color).toContain('green')
		})

		it('should return correct color classes for mild severity', () => {
			const color = getSeverityColor('mild')
			expect(color).toContain('yellow')
		})

		it('should return correct color classes for moderate severity', () => {
			const color = getSeverityColor('moderate')
			expect(color).toContain('orange')
		})

		it('should return correct color classes for moderately_severe severity', () => {
			const color = getSeverityColor('moderately_severe')
			expect(color).toContain('red')
		})

		it('should return correct color classes for severe severity', () => {
			const color = getSeverityColor('severe')
			expect(color).toContain('red')
		})

		it('should return default gray color for unknown severity', () => {
			const color = getSeverityColor('unknown')
			expect(color).toContain('gray')
		})
	})

	describe('getSeverityBgColor', () => {
		it('should return background color class only', () => {
			const color = getSeverityBgColor('minimal')
			expect(color).toBe('bg-green-500')
			expect(color).not.toContain('text-')
			expect(color).not.toContain('border-')
		})

		it('should return correct background colors for all severity levels', () => {
			expect(getSeverityBgColor('minimal')).toBe('bg-green-500')
			expect(getSeverityBgColor('mild')).toBe('bg-yellow-500')
			expect(getSeverityBgColor('moderate')).toBe('bg-orange-500')
			expect(getSeverityBgColor('moderately_severe')).toBe('bg-red-500')
			expect(getSeverityBgColor('severe')).toBe('bg-red-600')
		})

		it('should return default gray for unknown severity', () => {
			const color = getSeverityBgColor('unknown')
			expect(color).toBe('bg-gray-500')
		})
	})

	describe('getSeverityRanges', () => {
		it('should return correct ranges for PHQ-9', () => {
			const ranges = getSeverityRanges('PHQ9', 27)
			expect(ranges).toHaveLength(5)
			expect(ranges[0]).toEqual({ level: 'minimal', min: 0, max: 4, color: 'bg-green-500' })
			expect(ranges[1]).toEqual({ level: 'mild', min: 5, max: 9, color: 'bg-yellow-500' })
			expect(ranges[2]).toEqual({ level: 'moderate', min: 10, max: 14, color: 'bg-orange-500' })
			expect(ranges[3]).toEqual({
				level: 'moderately_severe',
				min: 15,
				max: 19,
				color: 'bg-red-500',
			})
			expect(ranges[4]).toEqual({ level: 'severe', min: 20, max: 27, color: 'bg-red-600' })
		})

		it('should return correct ranges for GAD-7', () => {
			const ranges = getSeverityRanges('GAD7', 21)
			expect(ranges).toHaveLength(4)
			expect(ranges[0]).toEqual({ level: 'minimal', min: 0, max: 4, color: 'bg-green-500' })
			expect(ranges[1]).toEqual({ level: 'mild', min: 5, max: 9, color: 'bg-yellow-500' })
			expect(ranges[2]).toEqual({ level: 'moderate', min: 10, max: 14, color: 'bg-orange-500' })
			expect(ranges[3]).toEqual({ level: 'severe', min: 15, max: 21, color: 'bg-red-600' })
		})

		it('should return correct ranges for CES-D', () => {
			const ranges = getSeverityRanges('CESD', 60)
			expect(ranges).toHaveLength(4)
			expect(ranges[0]).toEqual({ level: 'minimal', min: 0, max: 15, color: 'bg-green-500' })
			expect(ranges[1]).toEqual({ level: 'mild', min: 16, max: 21, color: 'bg-yellow-500' })
			expect(ranges[2]).toEqual({ level: 'moderate', min: 22, max: 36, color: 'bg-orange-500' })
			expect(ranges[3]).toEqual({ level: 'severe', min: 37, max: 60, color: 'bg-red-600' })
		})

		it('should return correct ranges for OASIS', () => {
			const ranges = getSeverityRanges('OASIS', 20)
			expect(ranges).toHaveLength(3)
			expect(ranges[0]).toEqual({ level: 'minimal', min: 0, max: 7, color: 'bg-green-500' })
			expect(ranges[1]).toEqual({ level: 'moderate', min: 8, max: 14, color: 'bg-orange-500' })
			expect(ranges[2]).toEqual({ level: 'severe', min: 15, max: 20, color: 'bg-red-600' })
		})

		it('should return default range for unknown assessment type', () => {
			const ranges = getSeverityRanges('UNKNOWN', 100)
			expect(ranges).toHaveLength(1)
			expect(ranges[0]).toEqual({ level: 'minimal', min: 0, max: 100, color: 'bg-gray-500' })
		})
	})

	describe('formatSeverity', () => {
		it('should format single word severity levels', () => {
			expect(formatSeverity('minimal')).toBe('Minimal')
			expect(formatSeverity('mild')).toBe('Mild')
			expect(formatSeverity('moderate')).toBe('Moderate')
			expect(formatSeverity('severe')).toBe('Severe')
		})

		it('should format multi-word severity levels with underscores', () => {
			expect(formatSeverity('moderately_severe')).toBe('Moderately Severe')
		})

		it('should handle empty strings', () => {
			expect(formatSeverity('')).toBe('')
		})
	})

	describe('calculateSegmentWidth', () => {
		it('should calculate correct width percentage for PHQ-9 minimal range', () => {
			const range = { level: 'minimal', min: 0, max: 4, color: 'bg-green-500' }
			const width = calculateSegmentWidth(range, 27)
			// Range 0-4 is 5 values out of 28 total (0-27 inclusive)
			expect(width).toBeCloseTo((5 / 28) * 100, 2)
		})

		it('should calculate correct width percentage for GAD-7 severe range', () => {
			const range = { level: 'severe', min: 15, max: 21, color: 'bg-red-600' }
			const width = calculateSegmentWidth(range, 21)
			// Range 15-21 is 7 values out of 22 total (0-21 inclusive)
			expect(width).toBeCloseTo((7 / 22) * 100, 2)
		})

		it('should handle single-value ranges', () => {
			const range = { level: 'minimal', min: 0, max: 0, color: 'bg-green-500' }
			const width = calculateSegmentWidth(range, 10)
			// Range 0-0 is 1 value out of 11 total (0-10 inclusive)
			expect(width).toBeCloseTo((1 / 11) * 100, 2)
		})

		it('should calculate correct width for full range', () => {
			const range = { level: 'all', min: 0, max: 27, color: 'bg-gray-500' }
			const width = calculateSegmentWidth(range, 27)
			// Full range should be 100%
			expect(width).toBeCloseTo(100, 2)
		})
	})
})
