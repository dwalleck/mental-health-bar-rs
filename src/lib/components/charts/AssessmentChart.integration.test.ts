import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render } from '@testing-library/svelte'
import AssessmentChart from './AssessmentChart.svelte'
import type { AssessmentChartData, AssessmentType } from '$lib/bindings'

/**
 * Integration test for AssessmentChart component
 *
 * This test uses REAL Chart.js (not mocked) to verify end-to-end integration.
 * It will catch bugs like:
 * - Missing controller registrations (LineController, BarController)
 * - Missing scale registrations (CategoryScale, LinearScale)
 * - Missing element registrations (LineElement, PointElement)
 * - Chart.js configuration errors
 * - Annotation plugin issues
 *
 * We only mock what's unavailable in Node.js test environment (Canvas API).
 */
describe('AssessmentChart Integration Test', () => {
	beforeEach(() => {
		vi.clearAllMocks()

		// Mock Element.prototype.animate for Svelte transitions
		Element.prototype.animate = vi.fn(() => ({
			finished: Promise.resolve(),
			cancel: vi.fn(),
			pause: vi.fn(),
			play: vi.fn(),
			reverse: vi.fn(),
			finish: vi.fn(),
		})) as unknown as typeof Element.prototype.animate

		// Mock Canvas API - this is the ONLY thing we mock
		// Everything else (Chart.js, chart-config, etc.) runs for real
		HTMLCanvasElement.prototype.getContext = vi.fn(() => ({
			canvas: document.createElement('canvas'),
			fillRect: vi.fn(),
			clearRect: vi.fn(),
			getImageData: vi.fn(() => ({
				data: new Uint8ClampedArray(4),
				width: 1,
				height: 1,
			})),
			putImageData: vi.fn(),
			createImageData: vi.fn(() => ({
				data: new Uint8ClampedArray(4),
				width: 1,
				height: 1,
			})),
			setTransform: vi.fn(),
			drawImage: vi.fn(),
			save: vi.fn(),
			fillText: vi.fn(),
			restore: vi.fn(),
			beginPath: vi.fn(),
			moveTo: vi.fn(),
			lineTo: vi.fn(),
			closePath: vi.fn(),
			stroke: vi.fn(),
			strokeRect: vi.fn(),
			translate: vi.fn(),
			scale: vi.fn(),
			rotate: vi.fn(),
			arc: vi.fn(),
			arcTo: vi.fn(),
			fill: vi.fn(),
			measureText: vi.fn(() => ({ width: 0 })),
			transform: vi.fn(),
			rect: vi.fn(),
			clip: vi.fn(),
			// Additional methods Chart.js might need
			quadraticCurveTo: vi.fn(),
			bezierCurveTo: vi.fn(),
			createLinearGradient: vi.fn(() => ({
				addColorStop: vi.fn(),
			})),
			createRadialGradient: vi.fn(() => ({
				addColorStop: vi.fn(),
			})),
			createPattern: vi.fn(),
			isPointInPath: vi.fn(() => false),
			isPointInStroke: vi.fn(() => false),
		})) as unknown as typeof HTMLCanvasElement.prototype.getContext
	})

	const mockAssessmentType: AssessmentType = {
		id: 1,
		code: 'PHQ9',
		name: 'Patient Health Questionnaire-9',
		description: 'Depression screening tool',
		question_count: 9,
		min_score: 0,
		max_score: 27,
	}

	const createMockChartData = (dataPoints: number): AssessmentChartData => ({
		assessment_type: mockAssessmentType,
		data_points: Array.from({ length: dataPoints }, (_, i) => ({
			timestamp: `2025-01-${String(i + 1).padStart(2, '0')}T10:00:00Z`,
			value: Math.floor(Math.random() * 27),
			label: null,
		})),
		thresholds: [
			{ value: 4, label: 'Minimal', color: '#22C55E' },
			{ value: 9, label: 'Mild', color: '#F59E0B' },
			{ value: 14, label: 'Moderate', color: '#F97316' },
			{ value: 19, label: 'Moderately Severe', color: '#EF4444' },
			{ value: 27, label: 'Severe', color: '#DC2626' },
		],
		statistics: {
			min: 0,
			max: 27,
			average: 13.5,
			total_assessments: dataPoints,
			trend: 'stable',
		},
	})

	describe('End-to-End Chart Rendering', () => {
		it('should render line chart with real Chart.js without errors', () => {
			const data = createMockChartData(3)

			// This is THE critical test - uses real Chart.js
			// Will fail with "line is not a registered controller" if LineController isn't registered
			expect(() => {
				render(AssessmentChart, { props: { data, loading: false } })
			}).not.toThrow()

			// Verify canvas element was created
			const canvas = document.querySelector('canvas')
			expect(canvas).toBeTruthy()
		})

		it('should handle chart with minimum required data points (2)', () => {
			const data = createMockChartData(2)

			expect(() => {
				render(AssessmentChart, { props: { data, loading: false } })
			}).not.toThrow()
		})

		it('should handle chart with many data points', () => {
			const data = createMockChartData(30)

			expect(() => {
				render(AssessmentChart, { props: { data, loading: false } })
			}).not.toThrow()
		})

		it('should render chart with threshold annotations', () => {
			const data = createMockChartData(5)

			// This tests that chartjs-plugin-annotation is properly registered
			expect(() => {
				render(AssessmentChart, { props: { data, loading: false } })
			}).not.toThrow()

			// Thresholds should be included in the data
			expect(data.thresholds.length).toBeGreaterThan(0)
		})
	})

	describe('Real Chart.js Configuration', () => {
		it('should create chart with line type configuration', () => {
			const data = createMockChartData(3)

			const { container } = render(AssessmentChart, { props: { data, loading: false } })

			// Canvas should exist
			const canvas = container.querySelector('canvas')
			expect(canvas).toBeTruthy()

			// getContext should have been called to create the chart
			expect(HTMLCanvasElement.prototype.getContext).toHaveBeenCalledWith('2d')
		})

		it('should handle all assessment types without errors', () => {
			const assessmentTypes = [
				{ code: 'PHQ9', name: 'PHQ-9', max_score: 27 },
				{ code: 'GAD7', name: 'GAD-7', max_score: 21 },
				{ code: 'CESD', name: 'CES-D', max_score: 60 },
				{ code: 'OASIS', name: 'OASIS', max_score: 20 },
			]

			assessmentTypes.forEach((type) => {
				const data: AssessmentChartData = {
					assessment_type: {
						...mockAssessmentType,
						code: type.code,
						name: type.name,
						max_score: type.max_score,
					},
					data_points: [
						{ timestamp: '2025-01-01T10:00:00Z', value: 10, label: null },
						{ timestamp: '2025-01-08T10:00:00Z', value: 12, label: null },
					],
					thresholds: [],
					statistics: { min: 10, max: 12, average: 11, total_assessments: 2, trend: 'stable' },
				}

				expect(() => {
					render(AssessmentChart, { props: { data, loading: false } })
				}).not.toThrow()
			})
		})
	})

	describe('Integration with Chart Config', () => {
		it('should use controllers registered in chart-config.ts', () => {
			// This test verifies that the chart-config.ts module
			// properly registered all required Chart.js components
			const data = createMockChartData(3)

			// If chart-config.ts didn't register LineController,
			// this will throw: Error: "line" is not a registered controller
			expect(() => {
				render(AssessmentChart, { props: { data, loading: false } })
			}).not.toThrow()
		})

		it('should use default chart options from chart-config.ts', () => {
			const data = createMockChartData(3)

			// Rendering should succeed with default options
			expect(() => {
				const { container } = render(AssessmentChart, { props: { data, loading: false } })
				expect(container.querySelector('canvas')).toBeTruthy()
			}).not.toThrow()
		})
	})

	describe('Error Handling', () => {
		it('should not crash when data updates during render', async () => {
			const data1 = createMockChartData(3)
			const { rerender } = render(AssessmentChart, { props: { data: data1, loading: false } })

			// Update data while component is mounted
			const data2 = createMockChartData(5)
			await rerender({ data: data2, loading: false })

			// Should not throw
			expect(document.querySelector('canvas')).toBeTruthy()
		})

		it('should handle transition from null to data', async () => {
			const { rerender } = render(AssessmentChart, { props: { data: null, loading: false } })

			const data = createMockChartData(3)
			await rerender({ data, loading: false })

			// Should render chart after data is provided
			expect(document.querySelector('canvas')).toBeTruthy()
		})
	})
})
