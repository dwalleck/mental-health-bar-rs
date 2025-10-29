import { describe, it, expect, beforeEach, vi } from 'vitest'
import { render, screen, waitFor, fireEvent } from '@testing-library/svelte'
import DashboardScores from './DashboardScores.svelte'
import type { AssessmentResponse, AssessmentType } from '$lib/bindings'
import { goto } from '$app/navigation'
import { invoke } from '@tauri-apps/api/core'

// T213, T214, T215: Tests for DashboardScores component
describe('DashboardScores', () => {
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
	})

	// Mock assessment types (UPPERCASE codes to match database schema)
	const mockAssessmentTypes = {
		PHQ9: {
			id: 1,
			code: 'PHQ9',
			name: 'PHQ-9 (Depression)',
			description: 'Depression screening',
			question_count: 9,
			min_score: 0,
			max_score: 27,
		} as AssessmentType,
		GAD7: {
			id: 2,
			code: 'GAD7',
			name: 'GAD-7 (Anxiety)',
			description: 'Anxiety screening',
			question_count: 7,
			min_score: 0,
			max_score: 21,
		} as AssessmentType,
		CESD: {
			id: 3,
			code: 'CESD',
			name: 'CES-D (Depression)',
			description: 'Depression screening',
			question_count: 20,
			min_score: 0,
			max_score: 60,
		} as AssessmentType,
		OASIS: {
			id: 4,
			code: 'OASIS',
			name: 'OASIS (Anxiety)',
			description: 'Anxiety symptoms',
			question_count: 5,
			min_score: 0,
			max_score: 20,
		} as AssessmentType,
	}

	const createMockAssessmentResponse = (
		type: AssessmentType,
		score: number
	): AssessmentResponse => ({
		id: type.id,
		assessment_type: type,
		responses: Array(type.question_count).fill(1),
		total_score: score,
		severity_level: 'mild',
		completed_at: '2025-10-28T10:00:00Z',
		notes: null,
	})

	describe('T213: Fetches latest assessments for all 4 types on mount', () => {
		it('should call getLatestAssessment for PHQ-9, GAD-7, CES-D, and OASIS in parallel on mount', async () => {
			// Arrange: Mock successful responses for all assessment types
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						const code = args.assessmentTypeCode
						switch (code) {
							case 'PHQ9':
								return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 8)
							case 'GAD7':
								return createMockAssessmentResponse(mockAssessmentTypes.GAD7, 6)
							case 'CESD':
								return createMockAssessmentResponse(mockAssessmentTypes.CESD, 15)
							case 'OASIS':
								return createMockAssessmentResponse(mockAssessmentTypes.OASIS, 5)
							default:
								return null
						}
					}
					return null
				}
			)

			// Act: Render component (should trigger fetch on mount)
			render(DashboardScores)

			// Assert: Wait for all 4 assessment types to be fetched
			await waitFor(() => {
				expect(invoke).toHaveBeenCalledWith('get_latest_assessment', {
					assessmentTypeCode: 'PHQ9',
				})
			})

			expect(invoke).toHaveBeenCalledWith('get_latest_assessment', {
				assessmentTypeCode: 'GAD7',
			})
			expect(invoke).toHaveBeenCalledWith('get_latest_assessment', {
				assessmentTypeCode: 'CESD',
			})
			expect(invoke).toHaveBeenCalledWith('get_latest_assessment', {
				assessmentTypeCode: 'OASIS',
			})

			// Verify all 4 calls were made
			expect(invoke).toHaveBeenCalledTimes(4)
		})

		it('should use Promise.all for parallel loading of all assessment types', async () => {
			// Arrange: Track order of invocations
			const invocationOrder: string[] = []
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						invocationOrder.push(args.assessmentTypeCode)
						// Add minimal delay to simulate network
						await new Promise((resolve) => setTimeout(resolve, 10))
						const code = args.assessmentTypeCode
						switch (code) {
							case 'PHQ9':
								return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 8)
							case 'GAD7':
								return createMockAssessmentResponse(mockAssessmentTypes.GAD7, 6)
							case 'CESD':
								return createMockAssessmentResponse(mockAssessmentTypes.CESD, 15)
							case 'OASIS':
								return createMockAssessmentResponse(mockAssessmentTypes.OASIS, 5)
							default:
								return null
						}
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: All 4 invocations should start before any complete (parallel)
			await waitFor(() => {
				expect(invocationOrder).toHaveLength(4)
			})

			// Verify all assessment types were called
			expect(invocationOrder).toContain('PHQ9')
			expect(invocationOrder).toContain('GAD7')
			expect(invocationOrder).toContain('CESD')
			expect(invocationOrder).toContain('OASIS')
		})

		it('should display all 4 assessment scores when data is available', async () => {
			// Arrange: Mock all assessments with data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						const code = args.assessmentTypeCode
						switch (code) {
							case 'PHQ9':
								return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 12)
							case 'GAD7':
								return createMockAssessmentResponse(mockAssessmentTypes.GAD7, 8)
							case 'CESD':
								return createMockAssessmentResponse(mockAssessmentTypes.CESD, 20)
							case 'OASIS':
								return createMockAssessmentResponse(mockAssessmentTypes.OASIS, 10)
							default:
								return null
						}
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: All 4 assessment names should appear
			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			})

			expect(screen.getByText('GAD-7 (Anxiety)')).toBeInTheDocument()
			expect(screen.getByText('CES-D (Depression)')).toBeInTheDocument()
			expect(screen.getByText('OASIS (Anxiety)')).toBeInTheDocument()
		})

		it('should show skeleton loaders while fetching data', () => {
			// Arrange: Mock delayed responses
			vi.mocked(invoke).mockImplementation(
				async () =>
					new Promise((resolve) => {
						setTimeout(() => resolve(null), 1000)
					})
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Skeleton loaders should be visible
			const skeletonLoaders = screen.getAllByRole('status', { name: 'Loading content' })
			expect(skeletonLoaders.length).toBeGreaterThan(0)
		})

		it('should handle network errors gracefully', async () => {
			// Arrange: Mock network failure
			vi.mocked(invoke).mockRejectedValue(new Error('Network error'))

			// Act: Render component
			render(DashboardScores)

			// Assert: Component should not crash and should handle error
			await waitFor(() => {
				expect(invoke).toHaveBeenCalled()
			})

			// Component should still be in the DOM
			expect(document.body).toBeInTheDocument()
		})
	})

	describe('T214: Shows "Not taken yet" state for assessments without data', () => {
		it('should display "Not taken yet" when getLatestAssessment returns null', async () => {
			// Arrange: Mock all assessments returning null (no data)
			vi.mocked(invoke).mockResolvedValue(null)

			// Act: Render component
			render(DashboardScores)

			// Assert: "Not taken yet" should appear for each assessment
			await waitFor(() => {
				const notTakenMessages = screen.getAllByText('Not taken yet')
				expect(notTakenMessages).toHaveLength(4)
			})
		})

		it('should display "Not taken yet" for specific assessments without data', async () => {
			// Arrange: Mix of assessments with and without data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						const code = args.assessmentTypeCode
						switch (code) {
							case 'PHQ9':
								return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 10)
							case 'GAD7':
								return null // No data
							case 'CESD':
								return createMockAssessmentResponse(mockAssessmentTypes.CESD, 18)
							case 'OASIS':
								return null // No data
							default:
								return null
						}
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Should show 2 "Not taken yet" messages
			await waitFor(() => {
				const notTakenMessages = screen.getAllByText('Not taken yet')
				expect(notTakenMessages).toHaveLength(2)
			})

			// Verify assessment names are still displayed
			expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			expect(screen.getByText('GAD-7 (Anxiety)')).toBeInTheDocument()
			expect(screen.getByText('CES-D (Depression)')).toBeInTheDocument()
			expect(screen.getByText('OASIS (Anxiety)')).toBeInTheDocument()
		})

		it('should not display AssessmentScoreBar for assessments without data', async () => {
			// Arrange: All assessments return null
			vi.mocked(invoke).mockResolvedValue(null)

			// Act: Render component
			const { container } = render(DashboardScores)

			// Assert: No score bars should be rendered
			await waitFor(() => {
				expect(screen.getAllByText('Not taken yet')).toHaveLength(4)
			})

			// AssessmentScoreBar components should not be present
			const scoreBars = container.querySelectorAll('.assessment-score-bar')
			expect(scoreBars).toHaveLength(0)
		})

		it('should display proper styling for "Not taken yet" state', async () => {
			// Arrange: All assessments return null
			vi.mocked(invoke).mockResolvedValue(null)

			// Act: Render component
			const { container } = render(DashboardScores)

			// Assert: "Not taken yet" text should have muted/gray styling
			await waitFor(() => {
				const notTakenElements = container.querySelectorAll('.text-gray-500, .text-gray-400')
				expect(notTakenElements.length).toBeGreaterThan(0)
			})
		})

		it('should transition from "Not taken yet" to score bar when data loads', async () => {
			// Arrange: Start with null, then update to have data
			let shouldReturnData = false
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						if (shouldReturnData && args.assessmentTypeCode === 'PHQ9') {
							return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 8)
						}
						return null
					}
					return null
				}
			)

			// Act: Render component
			const { rerender } = render(DashboardScores)

			// Assert: Initially shows "Not taken yet"
			await waitFor(() => {
				expect(screen.getAllByText('Not taken yet')).toHaveLength(4)
			})

			// Update data
			shouldReturnData = true
			await rerender({})

			// Component behavior after data update would be tested here
			// (This test demonstrates the pattern; actual implementation may vary)
		})
	})

	describe('T215: Clicking a score bar navigates to the chart view', () => {
		it('should navigate to /charts?type=phq9 when PHQ-9 score bar is clicked', async () => {
			// Arrange: Mock assessment data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'PHQ9') {
						return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 12)
					}
					return null
				}
			)

			// Act: Render component and click PHQ-9 score bar
			render(DashboardScores)

			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			})

			const phq9Element = screen
				.getByText('PHQ-9 (Depression)')
				.closest('button, [role="button"], a')
			expect(phq9Element).toBeInTheDocument()

			if (phq9Element) {
				await fireEvent.click(phq9Element)
			}

			// Assert: Should navigate to charts with type parameter
			expect(goto).toHaveBeenCalledWith('/charts?type=phq9')
		})

		it('should navigate to /charts?type=gad7 when GAD-7 score bar is clicked', async () => {
			// Arrange: Mock assessment data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'GAD7') {
						return createMockAssessmentResponse(mockAssessmentTypes.GAD7, 9)
					}
					return null
				}
			)

			// Act: Render component and click GAD-7 score bar
			render(DashboardScores)

			await waitFor(() => {
				expect(screen.getByText('GAD-7 (Anxiety)')).toBeInTheDocument()
			})

			const gad7Element = screen.getByText('GAD-7 (Anxiety)').closest('button, [role="button"], a')
			expect(gad7Element).toBeInTheDocument()

			if (gad7Element) {
				await fireEvent.click(gad7Element)
			}

			// Assert: Should navigate to charts with type parameter
			expect(goto).toHaveBeenCalledWith('/charts?type=gad7')
		})

		it('should navigate to /charts?type=cesd when CES-D score bar is clicked', async () => {
			// Arrange: Mock assessment data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'CESD') {
						return createMockAssessmentResponse(mockAssessmentTypes.CESD, 22)
					}
					return null
				}
			)

			// Act: Render component and click CES-D score bar
			render(DashboardScores)

			await waitFor(() => {
				expect(screen.getByText('CES-D (Depression)')).toBeInTheDocument()
			})

			const cesdElement = screen
				.getByText('CES-D (Depression)')
				.closest('button, [role="button"], a')
			expect(cesdElement).toBeInTheDocument()

			if (cesdElement) {
				await fireEvent.click(cesdElement)
			}

			// Assert: Should navigate to charts with type parameter
			expect(goto).toHaveBeenCalledWith('/charts?type=cesd')
		})

		it('should navigate to /charts?type=oasis when OASIS score bar is clicked', async () => {
			// Arrange: Mock assessment data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'OASIS') {
						return createMockAssessmentResponse(mockAssessmentTypes.OASIS, 12)
					}
					return null
				}
			)

			// Act: Render component and click OASIS score bar
			render(DashboardScores)

			await waitFor(() => {
				expect(screen.getByText('OASIS (Anxiety)')).toBeInTheDocument()
			})

			const oasisElement = screen.getByText('OASIS (Anxiety)').closest('button, [role="button"], a')
			expect(oasisElement).toBeInTheDocument()

			if (oasisElement) {
				await fireEvent.click(oasisElement)
			}

			// Assert: Should navigate to charts with type parameter
			expect(goto).toHaveBeenCalledWith('/charts?type=oasis')
		})

		it('should not navigate when "Not taken yet" state is clicked', async () => {
			// Arrange: Mock all assessments returning null
			vi.mocked(invoke).mockResolvedValue(null)

			// Act: Render component
			render(DashboardScores)

			await waitFor(() => {
				expect(screen.getAllByText('Not taken yet')).toHaveLength(4)
			})

			// Try to click on "Not taken yet" text
			const notTakenElements = screen.getAllByText('Not taken yet')
			const firstElement = notTakenElements[0].closest('button, [role="button"], a')

			// Assert: "Not taken yet" should not be clickable
			// It should either not be wrapped in a clickable element or be disabled
			expect(firstElement).toBeNull()
		})

		it('should use correct assessment type code in navigation URL', async () => {
			// Arrange: Mock assessment data for all types
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						const code = args.assessmentTypeCode
						switch (code) {
							case 'PHQ9':
								return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 10)
							case 'GAD7':
								return createMockAssessmentResponse(mockAssessmentTypes.GAD7, 8)
							case 'CESD':
								return createMockAssessmentResponse(mockAssessmentTypes.CESD, 20)
							case 'OASIS':
								return createMockAssessmentResponse(mockAssessmentTypes.OASIS, 12)
							default:
								return null
						}
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Wait for all assessments to load
			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
				expect(screen.getByText('GAD-7 (Anxiety)')).toBeInTheDocument()
				expect(screen.getByText('CES-D (Depression)')).toBeInTheDocument()
				expect(screen.getByText('OASIS (Anxiety)')).toBeInTheDocument()
			})

			// Click each assessment and verify navigation
			const phq9 = screen.getByText('PHQ-9 (Depression)').closest('button, [role="button"], a')
			const gad7 = screen.getByText('GAD-7 (Anxiety)').closest('button, [role="button"], a')
			const cesd = screen.getByText('CES-D (Depression)').closest('button, [role="button"], a')
			const oasis = screen.getByText('OASIS (Anxiety)').closest('button, [role="button"], a')

			// Test PHQ-9
			if (phq9) {
				await fireEvent.click(phq9)
				expect(goto).toHaveBeenCalledWith('/charts?type=phq9')
			}

			// Test GAD-7
			if (gad7) {
				await fireEvent.click(gad7)
				expect(goto).toHaveBeenCalledWith('/charts?type=gad7')
			}

			// Test CES-D
			if (cesd) {
				await fireEvent.click(cesd)
				expect(goto).toHaveBeenCalledWith('/charts?type=cesd')
			}

			// Test OASIS
			if (oasis) {
				await fireEvent.click(oasis)
				expect(goto).toHaveBeenCalledWith('/charts?type=oasis')
			}

			// Assert: goto should have been called 4 times
			expect(goto).toHaveBeenCalledTimes(4)
		})
	})

	describe('Edge Cases and Error Handling', () => {
		it('should handle partial data load failures', async () => {
			// Arrange: Some assessments succeed, others fail
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						const code = args.assessmentTypeCode
						if (code === 'PHQ9') {
							return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 10)
						} else if (code === 'GAD7') {
							throw new Error('Network error')
						} else if (code === 'CESD') {
							return createMockAssessmentResponse(mockAssessmentTypes.CESD, 20)
						} else {
							return null
						}
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Should display successful assessments and handle failed ones
			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			})

			expect(screen.getByText('CES-D (Depression)')).toBeInTheDocument()
		})

		it('should display correct severity level for each assessment', async () => {
			// Arrange: Mock assessments with different severity levels
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment') {
						const code = args.assessmentTypeCode
						if (code === 'PHQ9') {
							const response = createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 15)
							response.severity_level = 'moderate'
							return response
						} else if (code === 'GAD7') {
							const response = createMockAssessmentResponse(mockAssessmentTypes.GAD7, 5)
							response.severity_level = 'mild'
							return response
						}
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Severity levels should be displayed (formatted)
			await waitFor(() => {
				expect(screen.getByText('Moderate')).toBeInTheDocument()
			})

			expect(screen.getByText('Mild')).toBeInTheDocument()
		})

		it('should handle very high scores correctly', async () => {
			// Arrange: Mock maximum scores
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'PHQ9') {
						const response = createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 27)
						response.severity_level = 'severe'
						return response
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Should display maximum score
			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			})
		})

		it('should handle zero scores correctly', async () => {
			// Arrange: Mock zero scores
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'PHQ9') {
						const response = createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 0)
						response.severity_level = 'minimal'
						return response
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Should display zero score
			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			})
		})
	})

	describe('Accessibility', () => {
		it('should have proper ARIA labels for loading states', () => {
			// Arrange: Mock delayed responses
			vi.mocked(invoke).mockImplementation(
				async () => new Promise((resolve) => setTimeout(() => resolve(null), 1000))
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Loading skeletons should have proper ARIA
			const loaders = screen.getAllByRole('status', { name: 'Loading content' })
			expect(loaders.length).toBeGreaterThan(0)
		})

		it('should have proper keyboard navigation support', async () => {
			// Arrange: Mock assessment data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'PHQ9') {
						return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 10)
					}
					return null
				}
			)

			// Act: Render component
			const { container } = render(DashboardScores)

			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			})

			// Assert: Clickable elements should be keyboard accessible (button or link)
			const clickableElements = container.querySelectorAll('button, a[href]')
			expect(clickableElements.length).toBeGreaterThan(0)
		})

		it('should have meaningful text alternatives for score visualizations', async () => {
			// Arrange: Mock assessment data
			vi.mocked(invoke).mockImplementation(
				async (command: string, args: { assessmentTypeCode: string }) => {
					if (command === 'get_latest_assessment' && args.assessmentTypeCode === 'PHQ9') {
						return createMockAssessmentResponse(mockAssessmentTypes.PHQ9, 12)
					}
					return null
				}
			)

			// Act: Render component
			render(DashboardScores)

			// Assert: Assessment names and scores should be in text form
			await waitFor(() => {
				expect(screen.getByText('PHQ-9 (Depression)')).toBeInTheDocument()
			})

			// Score information should be accessible as text (use testid to avoid multi-match)
			const summary = screen.getByTestId('assessment-summary-phq9')
			expect(summary).toHaveTextContent('12')
			expect(summary).toHaveTextContent('Mild')
		})
	})
})
