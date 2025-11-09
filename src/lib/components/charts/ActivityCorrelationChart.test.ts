import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen } from '@testing-library/svelte'
import ActivityCorrelationChart from './ActivityCorrelationChart.svelte'
import type { ActivityMoodData } from '$lib/bindings'

// NOTE: We no longer mock Chart.js to catch real integration bugs!
// We only mock the Canvas API which is unavailable in Node.js test environment

describe('ActivityCorrelationChart', () => {
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

	const mockActivityData: ActivityMoodData[] = [
		{
			activity: {
				id: 1,
				group_id: 1,
				name: 'Exercise',
				color: '#4CAF50',
				icon: '🏃',
				created_at: '2025-01-01T00:00:00Z',
				deleted_at: null,
			},
			average_mood: 4.5,
			data_points: [
				{ timestamp: '2025-01-01T10:00:00Z', value: 4, label: null },
				{ timestamp: '2025-01-02T10:00:00Z', value: 5, label: null },
			],
		},
		{
			activity: {
				id: 2,
				group_id: 1,
				name: 'Reading',
				color: '#2196F3',
				icon: '📚',
				created_at: '2025-01-01T00:00:00Z',
				deleted_at: null,
			},
			average_mood: 4.0,
			data_points: [{ timestamp: '2025-01-01T14:00:00Z', value: 4, label: null }],
		},
		{
			activity: {
				id: 3,
				group_id: 1,
				name: 'Work',
				color: '#FF9800',
				icon: '💼',
				created_at: '2025-01-01T00:00:00Z',
				deleted_at: null,
			},
			average_mood: 2.5,
			data_points: [
				{ timestamp: '2025-01-01T09:00:00Z', value: 2, label: null },
				{ timestamp: '2025-01-02T09:00:00Z', value: 3, label: null },
			],
		},
	]

	describe('Loading State', () => {
		it('should display loading spinner when loading is true', () => {
			render(ActivityCorrelationChart, { props: { data: [], loading: true } })

			const spinner = document.querySelector('.animate-spin')
			expect(spinner).toBeInTheDocument()
			expect(spinner).toHaveClass('border-green-600')
		})

		it('should not render chart canvas when loading', () => {
			render(ActivityCorrelationChart, { props: { data: [], loading: true } })

			expect(screen.queryByRole('img')).not.toBeInTheDocument()
		})

		it('should not display empty state when loading', () => {
			render(ActivityCorrelationChart, { props: { data: [], loading: true } })

			expect(screen.queryByText('No Activity Data')).not.toBeInTheDocument()
		})
	})

	describe('Empty State', () => {
		it('should display empty state when data array is empty', () => {
			render(ActivityCorrelationChart, { props: { data: [], loading: false } })

			expect(screen.getByText('No Activity Data')).toBeInTheDocument()
			expect(
				screen.getByText(/Log mood check-ins with activities to see which activities correlate/)
			).toBeInTheDocument()
		})

		it('should display empty state icon', () => {
			render(ActivityCorrelationChart, { props: { data: [], loading: false } })

			const icon = document.querySelector('.empty-state svg')
			expect(icon).toBeInTheDocument()
			expect(icon).toHaveClass('text-gray-400')
		})

		it('should display clipboard icon in empty state', () => {
			render(ActivityCorrelationChart, { props: { data: [], loading: false } })

			const icon = document.querySelector('.empty-state svg path')
			expect(icon).toBeInTheDocument()
			// Check for clipboard SVG path
			expect(icon?.getAttribute('d')).toContain('M9 5H7a2')
		})
	})

	describe('Chart Rendering with Data', () => {
		it('should render canvas when data array has activities', () => {
			const { container } = render(ActivityCorrelationChart, {
				props: { data: mockActivityData, loading: false },
			})

			const canvas = container.querySelector('canvas')
			expect(canvas).toBeInTheDocument()
		})

		it('should not display loading or empty state when data is valid', () => {
			render(ActivityCorrelationChart, {
				props: { data: mockActivityData, loading: false },
			})

			expect(screen.queryByText('No Activity Data')).not.toBeInTheDocument()
			expect(document.querySelector('.animate-spin')).not.toBeInTheDocument()
		})

		it('should render with single activity', () => {
			const singleActivity = [mockActivityData[0]]
			const { container } = render(ActivityCorrelationChart, {
				props: { data: singleActivity, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should render with many activities', () => {
			const manyActivities = [
				...mockActivityData,
				{
					activity: {
						id: 4,
						group_id: 1,
						name: 'Meditation',
						color: '#9C27B0',
						icon: '🧘',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 4.8,
					data_points: [{ timestamp: '2025-01-01T06:00:00Z', value: 5, label: null }],
				},
				{
					activity: {
						id: 5,
						group_id: 1,
						name: 'Socializing',
						color: '#E91E63',
						icon: '👥',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 4.2,
					data_points: [{ timestamp: '2025-01-01T18:00:00Z', value: 4, label: null }],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: manyActivities, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})

	describe('Data Sorting', () => {
		it('should sort activities by average mood (highest first)', () => {
			// The chart should sort the data before rendering
			// We verify the component renders without errors with unsorted data
			const unsortedData = [
				{
					activity: {
						id: 1,
						group_id: 1,
						name: 'Low Mood',
						color: '#EF4444',
						icon: '😞',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 2.0,
					data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 2, label: null }],
				},
				{
					activity: {
						id: 2,
						group_id: 1,
						name: 'High Mood',
						color: '#22C55E',
						icon: '😊',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 5.0,
					data_points: [{ timestamp: '2025-01-01T11:00:00Z', value: 5, label: null }],
				},
				{
					activity: {
						id: 3,
						group_id: 1,
						name: 'Medium Mood',
						color: '#F59E0B',
						icon: '😐',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.5,
					data_points: [{ timestamp: '2025-01-01T12:00:00Z', value: 3, label: null }],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: unsortedData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle activities with same mood rating', () => {
			const sameRatingData = [
				{
					activity: {
						id: 1,
						group_id: 1,
						name: 'Activity A',
						color: '#F59E0B',
						icon: 'A',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.0,
					data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null }],
				},
				{
					activity: {
						id: 2,
						group_id: 1,
						name: 'Activity B',
						color: '#F59E0B',
						icon: 'B',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.0,
					data_points: [{ timestamp: '2025-01-01T11:00:00Z', value: 3, label: null }],
				},
				{
					activity: {
						id: 3,
						group_id: 1,
						name: 'Activity C',
						color: '#F59E0B',
						icon: 'C',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.0,
					data_points: [{ timestamp: '2025-01-01T12:00:00Z', value: 3, label: null }],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: sameRatingData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})

	describe('Chart.js Integration', () => {
		it('should create Chart instance when data is valid without throwing errors', async () => {
			// This test now uses REAL Chart.js
			// If BarController isn't registered, this will throw:
			// Error: "bar" is not a registered controller

			expect(() => {
				render(ActivityCorrelationChart, {
					props: { data: mockActivityData, loading: false },
				})
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
				render(ActivityCorrelationChart, {
					props: { data: mockActivityData, loading: false },
				})
			}).not.toThrow()

			// Wait for effect
			await new Promise((resolve) => setTimeout(resolve, 100))
		})
	})

	describe('Component Structure', () => {
		it('should have correct container class', () => {
			const { container } = render(ActivityCorrelationChart, {
				props: { data: mockActivityData, loading: false },
			})

			const chartDiv = container.querySelector('.activity-correlation-chart')
			expect(chartDiv).toBeInTheDocument()
		})

		it('should wrap canvas in chart-container', () => {
			const { container } = render(ActivityCorrelationChart, {
				props: { data: mockActivityData, loading: false },
			})

			const chartContainer = container.querySelector('.chart-container')
			const canvas = container.querySelector('canvas')

			expect(chartContainer).toBeInTheDocument()
			expect(chartContainer).toContainElement(canvas)
		})
	})

	describe('Edge Cases', () => {
		it('should handle transition from loading to data', async () => {
			const { rerender } = render(ActivityCorrelationChart, {
				props: { data: [], loading: true },
			})

			expect(document.querySelector('.animate-spin')).toBeInTheDocument()

			await rerender({ data: mockActivityData, loading: false })

			expect(document.querySelector('.animate-spin')).not.toBeInTheDocument()
		})

		it('should handle transition from empty to data', async () => {
			const { rerender } = render(ActivityCorrelationChart, {
				props: { data: [], loading: false },
			})

			expect(screen.getByText('No Activity Data')).toBeInTheDocument()

			await rerender({ data: mockActivityData, loading: false })

			expect(screen.queryByText('No Activity Data')).not.toBeInTheDocument()
		})

		it('should handle transition from data to empty', async () => {
			const { rerender, container } = render(ActivityCorrelationChart, {
				props: { data: mockActivityData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()

			await rerender({ data: [], loading: false })

			expect(container.querySelector('canvas')).not.toBeInTheDocument()
			expect(screen.getByText('No Activity Data')).toBeInTheDocument()
		})

		it('should handle activities with extreme mood values', () => {
			const extremeData = [
				{
					activity: {
						id: 1,
						group_id: 1,
						name: 'Very Bad Activity',
						color: '#EF4444',
						icon: '😰',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 1.0,
					data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 1, label: null }],
				},
				{
					activity: {
						id: 2,
						group_id: 1,
						name: 'Very Good Activity',
						color: '#22C55E',
						icon: '😄',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 5.0,
					data_points: [{ timestamp: '2025-01-01T11:00:00Z', value: 5, label: null }],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: extremeData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle activities with fractional mood values', () => {
			const fractionalData = [
				{
					activity: {
						id: 1,
						group_id: 1,
						name: 'Activity 1',
						color: '#F59E0B',
						icon: '1️⃣',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.14159,
					data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null }],
				},
				{
					activity: {
						id: 2,
						group_id: 1,
						name: 'Activity 2',
						color: '#F97316',
						icon: '2️⃣',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 2.71828,
					data_points: [{ timestamp: '2025-01-01T11:00:00Z', value: 3, label: null }],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: fractionalData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle activities with zero check-ins', () => {
			const zeroCheckinsData = [
				{
					activity: {
						id: 1,
						group_id: 1,
						name: 'Activity',
						color: '#F59E0B',
						icon: '📌',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.0,
					data_points: [],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: zeroCheckinsData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle activities with very long names', () => {
			const longNameData = [
				{
					activity: {
						id: 1,
						group_id: 1,
						name: 'This is a very long activity name that might cause layout issues',
						color: '#F59E0B',
						icon: '📝',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.5,
					data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null }],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: longNameData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})

		it('should handle many activities (stress test)', () => {
			const manyActivities = Array.from({ length: 50 }, (_, i) => ({
				activity: {
					id: i + 1,
					group_id: 1,
					name: `Activity ${i + 1}`,
					color: '#F59E0B',
					icon: '📊',
					created_at: '2025-01-01T00:00:00Z',
					deleted_at: null,
				},
				average_mood: Math.random() * 4 + 1, // Random mood between 1-5
				data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 3, label: null }],
			}))

			const { container } = render(ActivityCorrelationChart, {
				props: { data: manyActivities, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})

	describe('Mood Color Mapping', () => {
		it('should handle all mood rating ranges', () => {
			const allRatingsData = [
				{
					activity: {
						id: 1,
						group_id: 1,
						name: 'Very Bad (1)',
						color: '#EF4444',
						icon: '😰',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 1.0,
					data_points: [{ timestamp: '2025-01-01T10:00:00Z', value: 1, label: null }],
				},
				{
					activity: {
						id: 2,
						group_id: 1,
						name: 'Bad (2)',
						color: '#F97316',
						icon: '😟',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 2.0,
					data_points: [{ timestamp: '2025-01-01T11:00:00Z', value: 2, label: null }],
				},
				{
					activity: {
						id: 3,
						group_id: 1,
						name: 'Neutral (3)',
						color: '#F59E0B',
						icon: '😐',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 3.0,
					data_points: [{ timestamp: '2025-01-01T12:00:00Z', value: 3, label: null }],
				},
				{
					activity: {
						id: 4,
						group_id: 1,
						name: 'Good (4)',
						color: '#84CC16',
						icon: '🙂',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 4.0,
					data_points: [{ timestamp: '2025-01-01T13:00:00Z', value: 4, label: null }],
				},
				{
					activity: {
						id: 5,
						group_id: 1,
						name: 'Very Good (5)',
						color: '#22C55E',
						icon: '😄',
						created_at: '2025-01-01T00:00:00Z',
						deleted_at: null,
					},
					average_mood: 5.0,
					data_points: [{ timestamp: '2025-01-01T14:00:00Z', value: 5, label: null }],
				},
			]

			const { container } = render(ActivityCorrelationChart, {
				props: { data: allRatingsData, loading: false },
			})

			expect(container.querySelector('canvas')).toBeInTheDocument()
		})
	})
})
