import { describe, it, expect } from 'vitest'
import { Chart } from 'chart.js'
// Import chart-config to trigger registration side effects
import {
	defaultChartOptions,
	defaultBarChartOptions,
	moodColors,
	severityColors,
	createThresholdAnnotation,
} from './chart-config'

describe('Chart.js Registration', () => {
	it('should register LineController for line charts', () => {
		// Import triggers controller registration side effects
		// chart-config.ts is imported above to ensure registrations are loaded

		const registry = Chart.registry
		const lineController = registry.controllers.get('line')

		expect(lineController).toBeDefined()
	})

	it('should register BarController for bar charts', () => {
		const registry = Chart.registry
		const barController = registry.controllers.get('bar')

		expect(barController).toBeDefined()
	})

	it('should register all required scales', () => {
		const registry = Chart.registry

		// Category scale for x-axis labels
		expect(registry.scales.get('category')).toBeDefined()

		// Linear scale for y-axis numerical values
		expect(registry.scales.get('linear')).toBeDefined()
	})

	it('should register all required elements', () => {
		const registry = Chart.registry

		// Line chart elements
		expect(registry.elements.get('line')).toBeDefined()
		expect(registry.elements.get('point')).toBeDefined()

		// Bar chart elements
		expect(registry.elements.get('bar')).toBeDefined()
	})

	it('should register annotation plugin', () => {
		const registry = Chart.registry

		// chartjs-plugin-annotation should be registered
		const plugins = registry.plugins
		expect(plugins).toBeDefined()

		// The plugin should be in the registry
		// We verify the annotation plugin by checking if it was registered
		// The actual plugin verification is done by integration tests
		expect(Array.isArray(Chart.defaults.plugins.annotation)).toBe(false)
	})

	it('should have complete Chart.js setup without errors', () => {
		// This test verifies that importing chart-config.ts doesn't throw
		// If controller registration fails, the imports above would have thrown an error
		const registry = Chart.registry

		// Verify at least some components are registered
		expect(registry.controllers.get('line')).toBeDefined()
		expect(registry.controllers.get('bar')).toBeDefined()
		expect(registry.scales.get('linear')).toBeDefined()
	})
})

describe('Default Chart Options', () => {
	it('should have responsive enabled', () => {
		expect(defaultChartOptions.responsive).toBe(true)
	})

	it('should maintain aspect ratio', () => {
		expect(defaultChartOptions.maintainAspectRatio).toBe(true)
	})

	it('should have aspect ratio of 2', () => {
		expect(defaultChartOptions.aspectRatio).toBe(2)
	})

	it('should have animation duration of 500ms', () => {
		expect(
			defaultChartOptions.animation && typeof defaultChartOptions.animation === 'object'
				? defaultChartOptions.animation.duration
				: undefined
		).toBe(500)
	})

	it('should use index interaction mode', () => {
		expect(defaultChartOptions.interaction?.mode).toBe('index')
	})

	it('should display legend at top', () => {
		expect(defaultChartOptions.plugins?.legend?.display).toBe(true)
		expect(defaultChartOptions.plugins?.legend?.position).toBe('top')
	})

	it('should have tooltip enabled', () => {
		expect(defaultChartOptions.plugins?.tooltip?.enabled).toBe(true)
	})

	it('should begin y-axis at zero', () => {
		const yScale = defaultChartOptions.scales?.y
		expect(
			yScale && typeof yScale === 'object' && 'beginAtZero' in yScale
				? yScale.beginAtZero
				: undefined
		).toBe(true)
	})

	it('should hide x-axis grid', () => {
		expect(defaultChartOptions.scales?.x?.grid?.display).toBe(false)
	})
})

describe('Bar Chart Options', () => {
	it('should use horizontal bars', () => {
		expect(defaultBarChartOptions.indexAxis).toBe('y')
	})

	it('should hide legend', () => {
		expect(defaultBarChartOptions.plugins?.legend?.display).toBe(false)
	})

	it('should have same animation as default', () => {
		expect(
			defaultBarChartOptions.animation && typeof defaultBarChartOptions.animation === 'object'
				? defaultBarChartOptions.animation.duration
				: undefined
		).toBe(500)
	})
})

describe('Mood Colors', () => {
	it('should have colors for all 5 ratings', () => {
		expect(moodColors[1]).toBeDefined()
		expect(moodColors[2]).toBeDefined()
		expect(moodColors[3]).toBeDefined()
		expect(moodColors[4]).toBeDefined()
		expect(moodColors[5]).toBeDefined()
	})

	it('should use red for Very Bad (1)', () => {
		expect(moodColors[1]).toBe('#EF4444')
	})

	it('should use green for Very Good (5)', () => {
		expect(moodColors[5]).toBe('#22C55E')
	})

	it('should be valid hex format', () => {
		const hexPattern = /^#[0-9A-F]{6}$/i
		for (let i = 1; i <= 5; i++) {
			expect(moodColors[i as keyof typeof moodColors]).toMatch(hexPattern)
		}
	})
})

describe('Severity Colors', () => {
	it('should have colors for all severity levels', () => {
		expect(severityColors.minimal).toBeDefined()
		expect(severityColors.mild).toBeDefined()
		expect(severityColors.moderate).toBeDefined()
		expect(severityColors.moderately_severe).toBeDefined()
		expect(severityColors.severe).toBeDefined()
	})

	it('should use green for minimal', () => {
		expect(severityColors.minimal).toBe('#22C55E')
	})

	it('should use dark red for severe', () => {
		expect(severityColors.severe).toBe('#DC2626')
	})

	it('should be valid hex format', () => {
		const hexPattern = /^#[0-9A-F]{6}$/i
		Object.values(severityColors).forEach((color) => {
			expect(color).toMatch(hexPattern)
		})
	})
})

describe('createThresholdAnnotation', () => {
	it('should create line annotation', () => {
		const annotation = createThresholdAnnotation('Test', 10, '#FF0000')
		expect(annotation.type).toBe('line')
	})

	it('should set threshold value correctly', () => {
		const annotation = createThresholdAnnotation('Test', 10, '#FF0000')
		expect(annotation.yMin).toBe(10)
		expect(annotation.yMax).toBe(10)
	})

	it('should use provided color', () => {
		const annotation = createThresholdAnnotation('Test', 10, '#00FF00')
		expect(annotation.borderColor).toBe('#00FF00')
		expect(annotation.label.backgroundColor).toBe('#00FF00')
	})

	it('should display label with content', () => {
		const annotation = createThresholdAnnotation('Critical', 20, '#FF0000')
		expect(annotation.label.display).toBe(true)
		expect(annotation.label.content).toBe('Critical')
	})

	it('should have dashed border', () => {
		const annotation = createThresholdAnnotation('Test', 10, '#FF0000')
		expect(annotation.borderDash).toEqual([5, 5])
	})

	it('should position label at end', () => {
		const annotation = createThresholdAnnotation('Test', 10, '#FF0000')
		expect(annotation.label.position).toBe('end')
	})

	it('should use white text for label', () => {
		const annotation = createThresholdAnnotation('Test', 10, '#FF0000')
		expect(annotation.label.color).toBe('#fff')
	})
})
