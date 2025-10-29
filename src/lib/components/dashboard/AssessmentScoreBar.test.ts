import { describe, it, expect, beforeEach } from 'vitest'
import { render, screen } from '@testing-library/svelte'
import AssessmentScoreBar from './AssessmentScoreBar.svelte'
import type { AssessmentType } from '$lib/bindings'

describe('AssessmentScoreBar', () => {
	const mockAssessmentTypes: Record<string, AssessmentType> = {
		PHQ9: {
			id: 1,
			code: 'PHQ9',
			name: 'PHQ-9 (Depression)',
			description: 'Depression screening',
			question_count: 9,
			min_score: 0,
			max_score: 27,
		},
		GAD7: {
			id: 2,
			code: 'GAD7',
			name: 'GAD-7 (Anxiety)',
			description: 'Anxiety screening',
			question_count: 7,
			min_score: 0,
			max_score: 21,
		},
		CESD: {
			id: 3,
			code: 'CESD',
			name: 'CES-D (Depression)',
			description: 'Center for Epidemiologic Studies Depression Scale',
			question_count: 20,
			min_score: 0,
			max_score: 60,
		},
		OASIS: {
			id: 4,
			code: 'OASIS',
			name: 'OASIS (Anxiety)',
			description: 'Overall Anxiety Severity and Impairment Scale',
			question_count: 5,
			min_score: 0,
			max_score: 20,
		},
	}

	beforeEach(() => {
		// Mock any browser APIs if needed
	})

	describe('T211: Progress bar rendering with score value', () => {
		it('should render progress bar with score value for PHQ-9', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			// Check for progress bar element
			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// Check score is displayed
			expect(screen.getByText(/10/)).toBeInTheDocument()
		})

		it('should render progress bar with score value for GAD-7', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.GAD7,
					score: 8,
					severityLevel: 'mild',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			expect(screen.getByText(/8/)).toBeInTheDocument()
		})

		it('should render progress bar with score value for CES-D', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.CESD,
					score: 22,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			expect(screen.getByText(/22/)).toBeInTheDocument()
		})

		it('should render progress bar with score value for OASIS', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.OASIS,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			expect(screen.getByText(/10/)).toBeInTheDocument()
		})

		it('should display max score from assessment type', () => {
			render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			// Should display max score (27 for PHQ-9)
			expect(screen.getByText(/27/)).toBeInTheDocument()
		})

		it('should calculate correct percentage for progress bar', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 13,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// 13/27 * 100 ≈ 48.15%
			const ariaValueNow = progressBar?.getAttribute('aria-valuenow')
			expect(ariaValueNow).toBe('13')

			const ariaValueMax = progressBar?.getAttribute('aria-valuemax')
			expect(ariaValueMax).toBe('27')
		})

		it('should handle score at 0', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 0,
					severityLevel: 'minimal',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			expect(progressBar?.getAttribute('aria-valuenow')).toBe('0')
		})

		it('should handle score at maximum for PHQ-9', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 27,
					severityLevel: 'severe',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			expect(progressBar?.getAttribute('aria-valuenow')).toBe('27')
			expect(progressBar?.getAttribute('aria-valuemax')).toBe('27')
		})

		it('should handle score at maximum for GAD-7', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.GAD7,
					score: 21,
					severityLevel: 'severe',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar?.getAttribute('aria-valuenow')).toBe('21')
			expect(progressBar?.getAttribute('aria-valuemax')).toBe('21')
		})

		it('should have accessible aria labels', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			expect(progressBar?.getAttribute('aria-label')).toBeTruthy()
			expect(progressBar?.getAttribute('aria-valuenow')).toBe('10')
			expect(progressBar?.getAttribute('aria-valuemin')).toBe('0')
			expect(progressBar?.getAttribute('aria-valuemax')).toBe('27')
		})
	})

	describe('T212: Severity segments with correct colors', () => {
		it('should display severity segments for PHQ-9', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			// PHQ-9 has 5 severity levels: minimal (0-4), mild (5-9), moderate (10-14), moderately_severe (15-19), severe (20-27)
			const segments = container.querySelectorAll('.severity-segment')
			expect(segments.length).toBeGreaterThanOrEqual(4)
		})

		it('should display severity segments for GAD-7', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.GAD7,
					score: 8,
					severityLevel: 'mild',
				},
			})

			// GAD-7 has 4 severity levels: minimal (0-4), mild (5-9), moderate (10-14), severe (15-21)
			const segments = container.querySelectorAll('.severity-segment')
			expect(segments.length).toBeGreaterThanOrEqual(4)
		})

		it('should display severity segments for CES-D', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.CESD,
					score: 22,
					severityLevel: 'moderate',
				},
			})

			// CES-D has 4 severity levels: minimal (0-15), mild (16-21), moderate (22-36), severe (37-60)
			const segments = container.querySelectorAll('.severity-segment')
			expect(segments.length).toBeGreaterThanOrEqual(4)
		})

		it('should display severity segments for OASIS', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.OASIS,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			// OASIS has 3 severity levels: minimal (0-7), moderate (8-14), severe (15-20)
			const segments = container.querySelectorAll('.severity-segment')
			expect(segments.length).toBeGreaterThanOrEqual(3)
		})

		it('should apply minimal severity color (green)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 2,
					severityLevel: 'minimal',
				},
			})

			// Check for green color classes (matching AssessmentResults.svelte)
			const segments = container.querySelectorAll('.severity-segment')
			const minimalSegment = Array.from(segments).find((seg) => seg.className.includes('green'))
			expect(minimalSegment).toBeInTheDocument()
		})

		it('should apply mild severity color (yellow)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 7,
					severityLevel: 'mild',
				},
			})

			// Check for yellow color classes
			const segments = container.querySelectorAll('.severity-segment')
			const mildSegment = Array.from(segments).find((seg) => seg.className.includes('yellow'))
			expect(mildSegment).toBeInTheDocument()
		})

		it('should apply moderate severity color (orange)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 12,
					severityLevel: 'moderate',
				},
			})

			// Check for orange color classes
			const segments = container.querySelectorAll('.severity-segment')
			const moderateSegment = Array.from(segments).find((seg) => seg.className.includes('orange'))
			expect(moderateSegment).toBeInTheDocument()
		})

		it('should apply moderately severe severity color (red) for PHQ-9', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 17,
					severityLevel: 'moderately_severe',
				},
			})

			// Check for red color classes
			const segments = container.querySelectorAll('.severity-segment')
			const severeSegment = Array.from(segments).find((seg) => seg.className.includes('red'))
			expect(severeSegment).toBeInTheDocument()
		})

		it('should apply severe severity color (dark red)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 24,
					severityLevel: 'severe',
				},
			})

			// Check for dark red color classes
			const segments = container.querySelectorAll('.severity-segment')
			const severeSegment = Array.from(segments).find((seg) => seg.className.includes('red'))
			expect(severeSegment).toBeInTheDocument()
		})

		it('should highlight current severity level segment', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 12,
					severityLevel: 'moderate',
				},
			})

			// Current severity segment should have active/highlight styling
			const segments = container.querySelectorAll('.severity-segment')
			const activeSegment = Array.from(segments).find(
				(seg) =>
					seg.className.includes('active') ||
					seg.className.includes('current') ||
					seg.className.includes('highlight')
			)
			expect(activeSegment).toBeInTheDocument()
		})

		it('should display severity labels on segments', () => {
			render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			// Check for common severity labels
			const container = document.body
			expect(
				container.textContent?.includes('minimal') ||
					container.textContent?.includes('Minimal') ||
					container.textContent?.includes('MINIMAL')
			).toBe(true)
		})

		it('should handle score at minimal boundary (PHQ-9: score 4)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 4,
					severityLevel: 'minimal',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// Should display green for minimal
			const segments = container.querySelectorAll('.severity-segment')
			expect(segments.length).toBeGreaterThan(0)
		})

		it('should handle score at mild boundary (PHQ-9: score 5)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 5,
					severityLevel: 'mild',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// Should display yellow for mild
			const segments = container.querySelectorAll('.severity-segment')
			const mildSegment = Array.from(segments).find((seg) => seg.className.includes('yellow'))
			expect(mildSegment).toBeInTheDocument()
		})

		it('should handle score at moderate boundary (PHQ-9: score 10)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// Should display orange for moderate
			const segments = container.querySelectorAll('.severity-segment')
			const moderateSegment = Array.from(segments).find((seg) => seg.className.includes('orange'))
			expect(moderateSegment).toBeInTheDocument()
		})

		it('should handle score at moderately severe boundary (PHQ-9: score 15)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 15,
					severityLevel: 'moderately_severe',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// Should display red for moderately severe
			const segments = container.querySelectorAll('.severity-segment')
			const severeSegment = Array.from(segments).find((seg) => seg.className.includes('red'))
			expect(severeSegment).toBeInTheDocument()
		})

		it('should handle score at severe boundary (PHQ-9: score 20)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 20,
					severityLevel: 'severe',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// Should display dark red for severe
			const segments = container.querySelectorAll('.severity-segment')
			const severeSegment = Array.from(segments).find((seg) => seg.className.includes('red'))
			expect(severeSegment).toBeInTheDocument()
		})

		it('should handle GAD-7 moderate boundary (score 10)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.GAD7,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			const segments = container.querySelectorAll('.severity-segment')
			const moderateSegment = Array.from(segments).find((seg) => seg.className.includes('orange'))
			expect(moderateSegment).toBeInTheDocument()
		})

		it('should handle CES-D mild boundary (score 16)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.CESD,
					score: 16,
					severityLevel: 'mild',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			const segments = container.querySelectorAll('.severity-segment')
			const mildSegment = Array.from(segments).find((seg) => seg.className.includes('yellow'))
			expect(mildSegment).toBeInTheDocument()
		})

		it('should handle OASIS moderate boundary (score 8)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.OASIS,
					score: 8,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			const segments = container.querySelectorAll('.severity-segment')
			const moderateSegment = Array.from(segments).find((seg) => seg.className.includes('orange'))
			expect(moderateSegment).toBeInTheDocument()
		})
	})

	describe('Component Structure and Styling', () => {
		it('should have proper container class', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			const scoreBarContainer = container.querySelector('.assessment-score-bar')
			expect(scoreBarContainer).toBeInTheDocument()
		})

		it('should be responsive and use proper styling', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()

			// Should have width style based on score percentage
			// Score 10 out of 27 = ~37%
			expect(progressBar).toHaveAttribute('style', expect.stringContaining('width'))
		})

		it('should display assessment type name', () => {
			render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			// Should show PHQ-9 name
			expect(screen.getByText(/PHQ-9/)).toBeInTheDocument()
		})
	})

	describe('Edge Cases', () => {
		it('should handle invalid score (above max)', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 30,
					severityLevel: 'severe',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			// Should clamp to max or handle gracefully
		})

		it('should handle negative score', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: -5,
					severityLevel: 'minimal',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			// Should clamp to 0 or handle gracefully
		})

		it('should handle missing severity level', () => {
			const { container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'unknown',
				},
			})

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar).toBeInTheDocument()
			// Should handle unknown severity gracefully
		})

		it('should update when score changes', async () => {
			const { rerender } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			expect(screen.getByText(/10/)).toBeInTheDocument()

			await rerender({
				assessmentType: mockAssessmentTypes.PHQ9,
				score: 15,
				severityLevel: 'moderately_severe',
			})

			expect(screen.getByText(/15/)).toBeInTheDocument()
		})

		it('should update when assessment type changes', async () => {
			const { rerender, container } = render(AssessmentScoreBar, {
				props: {
					assessmentType: mockAssessmentTypes.PHQ9,
					score: 10,
					severityLevel: 'moderate',
				},
			})

			expect(screen.getByText(/PHQ-9/)).toBeInTheDocument()

			await rerender({
				assessmentType: mockAssessmentTypes.GAD7,
				score: 10,
				severityLevel: 'moderate',
			})

			expect(screen.getByText(/GAD-7/)).toBeInTheDocument()

			const progressBar = container.querySelector('[role="progressbar"]')
			expect(progressBar?.getAttribute('aria-valuemax')).toBe('21')
		})
	})
})
