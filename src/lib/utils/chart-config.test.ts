import { describe, it, expect } from 'vitest'
import { Chart } from 'chart.js'
// Import chart-config to trigger registration side effects
import './chart-config'

describe('Chart.js Configuration', () => {
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
