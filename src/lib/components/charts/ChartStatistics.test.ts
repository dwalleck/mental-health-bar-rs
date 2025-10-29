import { describe, it, expect } from 'vitest'
import { render, screen } from '@testing-library/svelte'
import ChartStatistics from './ChartStatistics.svelte'
import type { ChartStatistics as ChartStatisticsType } from '$lib/bindings'

describe('ChartStatistics', () => {
	const mockStatistics: ChartStatisticsType = {
		min: 5,
		max: 20,
		average: 12.5,
		total_assessments: 10,
		trend: 'improving',
	}

	describe('Rendering', () => {
		it('should render all statistics correctly', () => {
			render(ChartStatistics, { props: { statistics: mockStatistics } })

			expect(screen.getByText('Minimum')).toBeInTheDocument()
			expect(screen.getByText('5.0')).toBeInTheDocument()

			expect(screen.getByText('Maximum')).toBeInTheDocument()
			expect(screen.getByText('20.0')).toBeInTheDocument()

			expect(screen.getByText('Average')).toBeInTheDocument()
			expect(screen.getByText('12.5')).toBeInTheDocument()

			expect(screen.getByText('Total Assessments')).toBeInTheDocument()
			expect(screen.getByText('10')).toBeInTheDocument()
		})

		it('should render default title when not provided', () => {
			render(ChartStatistics, { props: { statistics: mockStatistics } })

			expect(screen.getByText('Statistics')).toBeInTheDocument()
		})

		it('should render custom title when provided', () => {
			render(ChartStatistics, {
				props: { statistics: mockStatistics, title: 'Assessment Statistics' },
			})

			expect(screen.getByText('Assessment Statistics')).toBeInTheDocument()
			expect(screen.queryByText('Statistics')).not.toBeInTheDocument()
		})

		it('should format decimal numbers to one decimal place', () => {
			const stats: ChartStatisticsType = {
				min: 5.678,
				max: 20.123,
				average: 12.567,
				total_assessments: 15,
				trend: 'stable',
			}

			render(ChartStatistics, { props: { statistics: stats } })

			expect(screen.getByText('5.7')).toBeInTheDocument()
			expect(screen.getByText('20.1')).toBeInTheDocument()
			expect(screen.getByText('12.6')).toBeInTheDocument()
		})

		it('should display total assessments as integer', () => {
			render(ChartStatistics, { props: { statistics: mockStatistics } })

			const totalElement = screen.getByText('10')
			expect(totalElement.textContent).toBe('10')
			expect(totalElement.textContent).not.toContain('.')
		})
	})

	describe('Trend Display', () => {
		it('should display improving trend correctly', () => {
			render(ChartStatistics, { props: { statistics: mockStatistics } })

			expect(screen.getByText('Trend')).toBeInTheDocument()
			expect(screen.getByText('Improving')).toBeInTheDocument()
			expect(screen.getByText('📉')).toBeInTheDocument()
		})

		it('should display worsening trend correctly', () => {
			const stats: ChartStatisticsType = { ...mockStatistics, trend: 'worsening' }
			render(ChartStatistics, { props: { statistics: stats } })

			expect(screen.getByText('Worsening')).toBeInTheDocument()
			expect(screen.getByText('📈')).toBeInTheDocument()
		})

		it('should display stable trend correctly', () => {
			const stats: ChartStatisticsType = { ...mockStatistics, trend: 'stable' }
			render(ChartStatistics, { props: { statistics: stats } })

			expect(screen.getByText('Stable')).toBeInTheDocument()
			expect(screen.getByText('➡️')).toBeInTheDocument()
		})

		it('should apply correct color class for improving trend', () => {
			render(ChartStatistics, { props: { statistics: mockStatistics } })

			const trendLabel = screen.getByText('Improving')
			expect(trendLabel).toHaveClass('text-green-600')
		})

		it('should apply correct color class for worsening trend', () => {
			const stats: ChartStatisticsType = { ...mockStatistics, trend: 'worsening' }
			render(ChartStatistics, { props: { statistics: stats } })

			const trendLabel = screen.getByText('Worsening')
			expect(trendLabel).toHaveClass('text-red-600')
		})

		it('should apply correct color class for stable trend', () => {
			const stats: ChartStatisticsType = { ...mockStatistics, trend: 'stable' }
			render(ChartStatistics, { props: { statistics: stats } })

			const trendLabel = screen.getByText('Stable')
			expect(trendLabel).toHaveClass('text-gray-600')
		})

		it('should handle unknown trend value gracefully', () => {
			const stats = {
				...mockStatistics,
				trend: 'unknown' as unknown as typeof mockStatistics.trend,
			}
			render(ChartStatistics, { props: { statistics: stats } })

			const trendLabel = screen.getByText('Unknown')
			expect(trendLabel).toBeInTheDocument()
			expect(trendLabel).toHaveClass('text-gray-600')
		})
	})

	describe('Layout Structure', () => {
		it('should render statistics in a grid layout', () => {
			const { container } = render(ChartStatistics, { props: { statistics: mockStatistics } })

			const grid = container.querySelector('.grid')
			expect(grid).toBeInTheDocument()
			expect(grid).toHaveClass('grid-cols-2')
		})

		it('should separate trend section with border', () => {
			const { container } = render(ChartStatistics, { props: { statistics: mockStatistics } })

			const trendSection = container.querySelector('.border-t')
			expect(trendSection).toBeInTheDocument()
		})

		it('should display trend icon and label together', () => {
			const { container } = render(ChartStatistics, { props: { statistics: mockStatistics } })

			const iconElement = screen.getByText('📉')
			const labelElement = screen.getByText('Improving')

			// Both should be in the same section
			const trendSection = container.querySelector('.border-t')
			expect(trendSection).toContainElement(iconElement)
			expect(trendSection).toContainElement(labelElement)
		})
	})

	describe('Edge Cases', () => {
		it('should handle zero values correctly', () => {
			const stats: ChartStatisticsType = {
				min: 0,
				max: 0,
				average: 0,
				total_assessments: 0,
				trend: 'stable',
			}

			render(ChartStatistics, { props: { statistics: stats } })

			// Should have three 0.0 values (min, max, average)
			const zeroDecimals = screen.getAllByText('0.0')
			expect(zeroDecimals).toHaveLength(3)

			// Should have one integer 0 (total_assessments)
			expect(screen.getByText('0')).toBeInTheDocument()
		})

		it('should handle very large numbers', () => {
			const stats: ChartStatisticsType = {
				min: 999.9,
				max: 9999.9,
				average: 5555.5,
				total_assessments: 10000,
				trend: 'improving',
			}

			render(ChartStatistics, { props: { statistics: stats } })

			expect(screen.getByText('999.9')).toBeInTheDocument()
			expect(screen.getByText('9999.9')).toBeInTheDocument()
			expect(screen.getByText('5555.5')).toBeInTheDocument()
			expect(screen.getByText('10000')).toBeInTheDocument()
		})

		it('should handle negative values', () => {
			const stats: ChartStatisticsType = {
				min: -10,
				max: -5,
				average: -7.5,
				total_assessments: 5,
				trend: 'worsening',
			}

			render(ChartStatistics, { props: { statistics: stats } })

			expect(screen.getByText('-10.0')).toBeInTheDocument()
			expect(screen.getByText('-5.0')).toBeInTheDocument()
			expect(screen.getByText('-7.5')).toBeInTheDocument()
		})
	})
})
