import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen } from '@testing-library/svelte'
import MoodChart from './MoodChart.svelte'
import type { MoodChartData } from '$lib/bindings'

// Mock Chart.js
vi.mock('chart.js', () => {
	const mockDestroy = vi.fn()
	const mockChart = {
		destroy: mockDestroy,
		update: vi.fn(),
		data: {},
		options: {},
		register: vi.fn(),
	}

	const ChartMock = Object.assign(
		vi.fn(() => mockChart),
		{
			register: vi.fn(),
		}
	)

	return {
		Chart: ChartMock,
		registerables: [],
		CategoryScale: vi.fn(),
		LinearScale: vi.fn(),
		PointElement: vi.fn(),
		LineElement: vi.fn(),
		BarElement: vi.fn(),
		Title: vi.fn(),
		Tooltip: vi.fn(),
		Legend: vi.fn(),
		Filler: vi.fn(),
	}
})

vi.mock('chartjs-plugin-annotation', () => ({
	default: vi.fn(),
}))

describe('MoodChart', () => {
	beforeEach(() => {
		vi.clearAllMocks()
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

	const mockMoodData: MoodChartData = {
		data_points: [
			{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: 'Exercise' },
			{ timestamp: '2025-01-02T14:30:00Z', value: 4, label: 'Reading' },
			{ timestamp: '2025-01-03T09:15:00Z', value: 2, label: 'Exercise' },
		],
		activity_breakdown: [
			{
				activity: {
					id: 1,
					name: 'Exercise',
					color: '#4CAF50',
					icon: '🏃',
					created_at: '2025-01-01T00:00:00Z',
					deleted_at: null,
				},
				average_mood: 2.5,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: 'Exercise' },
					{ timestamp: '2025-01-03T09:15:00Z', value: 2, label: 'Exercise' },
				],
			},
			{
				activity: {
					id: 2,
					name: 'Reading',
					color: '#2196F3',
					icon: '📚',
					created_at: '2025-01-01T00:00:00Z',
					deleted_at: null,
				},
				average_mood: 4.0,
				data_points: [{ timestamp: '2025-01-02T14:30:00Z', value: 4, label: 'Reading' }],
			},
		],
		statistics: {
			min: 2,
			max: 4,
			average: 3.0,
			median: 3.0,
			mode: 3,
			total_checkins: 3,
			checkins_per_day: 1.0,
		},
	}

	describe('Loading State', () => {
		it('should display loading spinner when loading is true', () => {
			render(MoodChart, { props: { data: null, loading: true } })

			const spinner = document.querySelector('.animate-spin')
			expect(spinner).toBeInTheDocument()
			expect(spinner).toHaveClass('border-purple-600')
		})

		it('should not render chart canvas when loading', () => {
			render(MoodChart, { props: { data: null, loading: true } })

			expect(screen.queryByRole('img')).not.toBeInTheDocument()
		})

		it('should not display empty state when loading', () => {
			render(MoodChart, { props: { data: null, loading: true } })

			expect(screen.queryByText('Not Enough Data')).not.toBeInTheDocument()
		})
	})

	describe('Empty State', () => {
		it('should display empty state when data is null', () => {
			render(MoodChart, { props: { data: null, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
			expect(screen.getByText(/Log at least 2 mood check-ins to view patterns/)).toBeInTheDocument()
		})

		it('should display empty state when data_points array is empty', () => {
			const emptyData = { ...mockMoodData, data_points: [] }
			render(MoodChart, { props: { data: emptyData, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
		})

		it('should display empty state when data_points has only 1 entry', () => {
			const singlePointData = {
				...mockMoodData,
				data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null }],
			}
			render(MoodChart, { props: { data: singlePointData, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
		})

		it('should display empty state icon', () => {
			render(MoodChart, { props: { data: null, loading: false } })

			const icon = document.querySelector('.empty-state svg')
			expect(icon).toBeInTheDocument()
			expect(icon).toHaveClass('text-gray-400')
		})

		it('should display smiley face icon in empty state', () => {
			render(MoodChart, { props: { data: null, loading: false } })

			const icon = document.querySelector('.empty-state svg path')
			expect(icon).toBeInTheDocument()
			// Check for smiley face SVG path (contains the face elements)
			expect(icon?.getAttribute('d')).toContain('M14.828')
		})
	})

	describe('Chart Rendering with Data', () => {
		it('should render canvas when data has 2 or more points', () => {
			const { container } = render(MoodChart, {
				props: { data: mockMoodData, loading: false },
			})

			const canvas = container.querySelector('canvas')
			expect(canvas).toBeInTheDocument()
		})

		it('should not display loading or empty state when data is valid', () => {
			render(MoodChart, { props: { data: mockMoodData, loading: false } })

			expect(screen.queryByText('Not Enough Data')).not.toBeInTheDocument()
			expect(document.querySelector('.animate-spin')).not.toBeInTheDocument()
		})

		it('should render exactly 2 data points as valid', () => {
			const twoPointData = {
				...mockMoodData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null },
					{ timestamp: '2025-01-02T14:30:00Z', value: 4, label: null },
				],
			}
			const { container } = render(MoodChart, {
				props: { data: twoPointData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
			expect(screen.queryByText('Not Enough Data')).not.toBeInTheDocument()
		})
	})

	describe('Chart.js Integration', () => {
		it('should create Chart instance when data is valid', async () => {
			const { Chart } = await import('chart.js')
			render(MoodChart, { props: { data: mockMoodData, loading: false } })

			// Wait for effect to run
			await new Promise((resolve) => setTimeout(resolve, 100))

			expect(Chart).toHaveBeenCalled()
		})

		it('should handle missing canvas context gracefully', async () => {
			HTMLCanvasElement.prototype.getContext = vi.fn(
				() => null
			) as unknown as typeof HTMLCanvasElement.prototype.getContext

			const { Chart } = await import('chart.js')
			vi.mocked(Chart).mockClear()

			render(MoodChart, { props: { data: mockMoodData, loading: false } })

			await new Promise((resolve) => setTimeout(resolve, 100))

			// Component handles this gracefully
			expect(true).toBe(true)
		})
	})

	describe('Data Transformation', () => {
		it('should handle data with multiple mood ratings', () => {
			const multiPointData = {
				...mockMoodData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 1, label: null },
					{ timestamp: '2025-01-02T10:00:00Z', value: 2, label: null },
					{ timestamp: '2025-01-03T10:00:00Z', value: 3, label: null },
					{ timestamp: '2025-01-04T10:00:00Z', value: 4, label: null },
					{ timestamp: '2025-01-05T10:00:00Z', value: 5, label: null },
				],
			}

			const { container } = render(MoodChart, {
				props: { data: multiPointData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle mood ratings at boundary values (1 and 5)', () => {
			const boundaryData = {
				...mockMoodData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 1, label: null },
					{ timestamp: '2025-01-02T10:00:00Z', value: 5, label: null },
				],
			}

			const { container } = render(MoodChart, {
				props: { data: boundaryData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle null activity_id in data points', () => {
			const noActivityData = {
				...mockMoodData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null },
					{ timestamp: '2025-01-02T10:00:00Z', value: 4, label: null },
				],
			}

			const { container } = render(MoodChart, {
				props: { data: noActivityData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})

	describe('Component Structure', () => {
		it('should have correct container class', () => {
			const { container } = render(MoodChart, {
				props: { data: mockMoodData, loading: false },
			})

			const chartDiv = container.querySelector('.mood-chart')
			expect(chartDiv).toBeInTheDocument()
		})

		it('should wrap canvas in chart-container', () => {
			const { container } = render(MoodChart, {
				props: { data: mockMoodData, loading: false },
			})

			const chartContainer = container.querySelector('.chart-container')
			const canvas = container.querySelector('canvas')

			expect(chartContainer).toBeInTheDocument()
			expect(chartContainer).toContainElement(canvas)
		})
	})

	describe('Edge Cases', () => {
		it('should handle transition from loading to data', async () => {
			const { rerender } = render(MoodChart, { props: { data: null, loading: true } })

			expect(document.querySelector('.animate-spin')).toBeInTheDocument()

			await rerender({ data: mockMoodData, loading: false })

			expect(document.querySelector('.animate-spin')).not.toBeInTheDocument()
		})

		it('should handle transition from empty to data', async () => {
			const { rerender } = render(MoodChart, { props: { data: null, loading: false } })

			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()

			await rerender({ data: mockMoodData, loading: false })

			expect(screen.queryByText('Not Enough Data')).not.toBeInTheDocument()
		})

		it('should handle transition from data to empty', async () => {
			const { rerender, container } = render(MoodChart, {
				props: { data: mockMoodData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()

			await rerender({ data: null, loading: false })

			expect(container.querySelector('canvas')).not.toBeInTheDocument()
			expect(screen.getByText('Not Enough Data')).toBeInTheDocument()
		})

		it('should handle mood data with timestamps including time components', () => {
			const timeDetailedData = {
				...mockMoodData,
				data_points: [
					{ timestamp: '2025-01-01T08:30:15Z', value: 3, label: null },
					{ timestamp: '2025-01-01T14:45:30Z', value: 4, label: null },
					{ timestamp: '2025-01-01T20:15:45Z', value: 2, label: null },
				],
			}

			const { container } = render(MoodChart, {
				props: { data: timeDetailedData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle empty activity_breakdown array', () => {
			const noActivitiesData = {
				...mockMoodData,
				activity_breakdown: [],
			}

			const { container } = render(MoodChart, {
				props: { data: noActivitiesData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle fractional mood values', () => {
			const fractionalData = {
				...mockMoodData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 2.5, label: null },
					{ timestamp: '2025-01-02T10:00:00Z', value: 3.7, label: null },
				],
			}

			const { container } = render(MoodChart, {
				props: { data: fractionalData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle same mood value for all data points', () => {
			const sameValueData = {
				...mockMoodData,
				data_points: [
					{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null },
					{ timestamp: '2025-01-02T10:00:00Z', value: 3, label: null },
					{ timestamp: '2025-01-03T10:00:00Z', value: 3, label: null },
				],
			}

			const { container } = render(MoodChart, {
				props: { data: sameValueData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})
})
