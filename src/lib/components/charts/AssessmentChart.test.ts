import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen } from '@testing-library/svelte'
import AssessmentChart from './AssessmentChart.svelte'
import type { AssessmentChartData } from '$lib/bindings'

/**
 * Week 0 Chart.js + Svelte 5 + Tauri Validation Suite (21 tests)
 *
 * This test suite validates the Chart.js integration with Svelte 5 runes
 * and the component's behavior across various states.
 *
 * NOTE: We no longer mock Chart.js to catch real integration bugs!
 * We only mock the Canvas API which is unavailable in Node.js test environment.
 */

describe('AssessmentChart', () => {
	beforeEach(() => {
		vi.clearAllMocks()

		// T181: Mock Element.prototype.animate for Svelte transitions
		Element.prototype.animate = vi.fn(() => ({
			finished: Promise.resolve(),
			cancel: vi.fn(),
			pause: vi.fn(),
			play: vi.fn(),
			reverse: vi.fn(),
			finish: vi.fn(),
		})) as unknown as typeof Element.prototype.animate

		// Mock canvas context - using type assertion for test mock
		HTMLCanvasElement.prototype.getContext = vi.fn(() => ({
			canvas: {},
			fillRect: vi.fn(),
			clearRect: vi.fn(),
			getImageData: vi.fn(),
			putImageData: vi.fn(),
			createImageData: vi.fn(),
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
			translate: vi.fn(),
			scale: vi.fn(),
			rotate: vi.fn(),
			arc: vi.fn(),
			fill: vi.fn(),
			measureText: vi.fn(() => ({ width: 0 })),
			transform: vi.fn(),
			rect: vi.fn(),
			clip: vi.fn(),
		})) as unknown as typeof HTMLCanvasElement.prototype.getContext
	})

	const mockAssessmentType = {
		id: 1,
		code: 'PHQ9' as const,
		name: 'PHQ-9 (Depression)',
		description: 'Depression screening',
		question_count: 9,
		min_score: 0,
		max_score: 27,
	}

	const mockChartData: AssessmentChartData = {
		assessment_type: mockAssessmentType,
		data_points: [
			{ timestamp: '2025-01-01T10:00:00Z', value: 10, label: null },
			{ timestamp: '2025-01-08T10:00:00Z', value: 8, label: null },
			{ timestamp: '2025-01-15T10:00:00Z', value: 6, label: null },
		],
		thresholds: [
			{ value: 5, label: 'Minimal', color: '#10B981' },
			{ value: 10, label: 'Mild', color: '#FBBF24' },
			{ value: 15, label: 'Moderate', color: '#F97316' },
			{ value: 20, label: 'Severe', color: '#EF4444' },
		],
		statistics: {
			min: 6,
			max: 10,
			average: 8,
			total_assessments: 3,
			trend: 'improving',
		},
	}

	describe('Loading State', () => {
		it('should display loading skeleton when loading is true', () => {
			render(AssessmentChart, { props: { data: null, loading: true } })

			// T181: Check for skeleton loader instead of spinner
			const loadingStatus = screen.getByRole('status')
			expect(loadingStatus).toBeInTheDocument()
			expect(loadingStatus).toHaveAttribute('aria-label', 'Loading content')
		})

		it('should not render chart canvas when loading', () => {
			render(AssessmentChart, { props: { data: null, loading: true } })

			expect(screen.queryByRole('img')).not.toBeInTheDocument()
		})
	})

	describe('Empty State', () => {
		it('should display empty state when data is null', () => {
			render(AssessmentChart, { props: { data: null, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
			expect(screen.getByText(/Complete at least 2 assessments to view trends/)).toBeInTheDocument()
		})

		it('should display empty state when data_points array is empty', () => {
			const emptyData = { ...mockChartData, data_points: [] }
			render(AssessmentChart, { props: { data: emptyData, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
		})

		it('should display empty state when data_points has only 1 entry', () => {
			const singlePointData = {
				...mockChartData,
				data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 10, label: null }],
			}
			render(AssessmentChart, { props: { data: singlePointData, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
		})

		it('should display empty state icon', () => {
			render(AssessmentChart, { props: { data: null, loading: false } })

			const icon = document.querySelector('.empty-state svg')
			expect(icon).toBeInTheDocument()
			expect(icon).toHaveClass('text-gray-400')
		})
	})

	describe('Chart Rendering with Data', () => {
		it('should render canvas when data has 2 or more points', () => {
			const { container } = render(AssessmentChart, {
				props: { data: mockChartData, loading: false },
			})

			const canvas = container.querySelector('canvas')
			expect(canvas).toBeInTheDocument()
		})

		it('should not display loading or empty state when data is valid', () => {
			render(AssessmentChart, { props: { data: mockChartData, loading: false } })

			expect(screen.queryByText('Not Enough Data')).not.toBeInTheDocument()
			expect(document.querySelector('.animate-spin')).not.toBeInTheDocument()
		})

		it('should render exactly 2 data points as valid', () => {
			const twoPointData = {
				...mockChartData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 10, label: null },
					{ timestamp: '2025-01-08T10:00:00Z', value: 8, label: null },
				],
			}
			const { container } = render(AssessmentChart, {
				props: { data: twoPointData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
			expect(screen.queryByText('Not Enough Data')).not.toBeInTheDocument()
		})
	})

	describe('Chart.js Integration', () => {
		it('should create Chart instance when data is valid without throwing errors', async () => {
			// This test now uses REAL Chart.js
			// If LineController isn't registered, this will throw:
			// Error: "line" is not a registered controller

			expect(() => {
				render(AssessmentChart, { props: { data: mockChartData, loading: false } })
			}).not.toThrow()

			// Wait for effect to run
			await new Promise((resolve) => setTimeout(resolve, 100))
		})

		it('should handle missing canvas context gracefully', async () => {
			HTMLCanvasElement.prototype.getContext = vi.fn(
				() => null
			) as unknown as typeof HTMLCanvasElement.prototype.getContext

			// Component should not throw even when canvas context is unavailable
			expect(() => {
				render(AssessmentChart, { props: { data: mockChartData, loading: false } })
			}).not.toThrow()

			// Wait for effect
			await new Promise((resolve) => setTimeout(resolve, 100))
		})
	})

	describe('Data Transformation', () => {
		it('should handle data with multiple timestamps', () => {
			const multiPointData = {
				...mockChartData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 15, label: null },
					{ timestamp: '2025-01-05T10:00:00Z', value: 12, label: null },
					{ timestamp: '2025-01-10T10:00:00Z', value: 10, label: null },
					{ timestamp: '2025-01-15T10:00:00Z', value: 8, label: null },
					{ timestamp: '2025-01-20T10:00:00Z', value: 6, label: null },
				],
			}

			const { container } = render(AssessmentChart, {
				props: { data: multiPointData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle threshold data correctly', () => {
			const dataWithThresholds = {
				...mockChartData,
				thresholds: [
					{ value: 10, label: 'Threshold 1', color: '#FF0000' },
					{ value: 20, label: 'Threshold 2', color: '#00FF00' },
				],
			}

			const { container } = render(AssessmentChart, {
				props: { data: dataWithThresholds, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle empty thresholds array', () => {
			const noThresholds = { ...mockChartData, thresholds: [] }

			const { container } = render(AssessmentChart, {
				props: { data: noThresholds, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})

	describe('Component Structure', () => {
		it('should have correct container class', () => {
			const { container } = render(AssessmentChart, {
				props: { data: mockChartData, loading: false },
			})

			const chartDiv = container.querySelector('.assessment-chart')
			expect(chartDiv).toBeInTheDocument()
		})

		it('should wrap canvas in chart-container', () => {
			const { container } = render(AssessmentChart, {
				props: { data: mockChartData, loading: false },
			})

			const chartContainer = container.querySelector('.chart-container')
			const canvas = container.querySelector('canvas')

			expect(chartContainer).toBeInTheDocument()
			expect(chartContainer).toContainElement(canvas)
		})
	})

	describe('Edge Cases', () => {
		it('should handle transition from loading to data', async () => {
			const { rerender } = render(AssessmentChart, { props: { data: null, loading: true } })

			// T181: Check for skeleton loader
			expect(screen.getByRole('status')).toBeInTheDocument()

			await rerender({ data: mockChartData, loading: false })

			expect(screen.queryByRole('status')).not.toBeInTheDocument()
		})

		it('should handle transition from empty to data', async () => {
			const { rerender } = render(AssessmentChart, { props: { data: null, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()

			await rerender({ data: mockChartData, loading: false })

			// T181: Element updates immediately without transitions
			expect(screen.queryByText('Not Enough Data')).not.toBeInTheDocument()
		})

		it('should handle transition from data to empty', async () => {
			const { rerender, container } = render(AssessmentChart, {
				props: { data: mockChartData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()

			await rerender({ data: null, loading: false })

			// T181: Element updates immediately without transitions
			expect(container.querySelector('canvas')).not.toBeInTheDocument()
			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
		})

		it('should handle data with very high scores', () => {
			const highScoreData = {
				...mockChartData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 25, label: null },
					{ timestamp: '2025-01-08T10:00:00Z', value: 27, label: null },
				],
			}

			const { container } = render(AssessmentChart, {
				props: { data: highScoreData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle data with zero scores', () => {
			const zeroScoreData = {
				...mockChartData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 0, label: null },
					{ timestamp: '2025-01-08T10:00:00Z', value: 0, label: null },
				],
			}

			const { container } = render(AssessmentChart, {
				props: { data: zeroScoreData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})
})
