// Component test for AssessmentForm
import { render, screen, waitFor } from '@testing-library/svelte';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import AssessmentForm from './AssessmentForm.svelte';
import type { AssessmentQuestion } from '$lib/bindings';

// Mock Tauri API
const mockInvoke = vi.fn();
vi.mock('@tauri-apps/api/core', () => ({
	invoke: mockInvoke
}));

// Mock SvelteKit navigation
const mockGoto = vi.fn();
vi.mock('$app/navigation', () => ({
	goto: mockGoto
}));

describe('AssessmentForm', () => {
	const mockQuestions: AssessmentQuestion[] = [
		{
			number: 1,
			text: 'Little interest or pleasure in doing things?',
			options: ['Not at all', 'Several days', 'More than half the days', 'Nearly every day']
		},
		{
			number: 2,
			text: 'Feeling down, depressed, or hopeless?',
			options: ['Not at all', 'Several days', 'More than half the days', 'Nearly every day']
		},
		{
			number: 3,
			text: 'Trouble falling or staying asleep, or sleeping too much?',
			options: ['Not at all', 'Several days', 'More than half the days', 'Nearly every day']
		}
	];

	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('renders loading state initially', () => {
		mockInvoke.mockImplementation(
			() => new Promise(() => {}) // Never resolves to keep loading state
		);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		expect(screen.getByText(/Loading assessment/i)).toBeInTheDocument();
	});

	it('fetches and renders assessment questions correctly', async () => {
		mockInvoke.mockResolvedValue(mockQuestions);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			expect(mockInvoke).toHaveBeenCalledWith('get_assessment_questions', {
				assessmentTypeCode: 'PHQ9'
			});
		});

		// Check that all questions are rendered
		await waitFor(() => {
			expect(screen.getByText(/Little interest or pleasure in doing things/i)).toBeInTheDocument();
			expect(screen.getByText(/Feeling down, depressed, or hopeless/i)).toBeInTheDocument();
			expect(
				screen.getByText(/Trouble falling or staying asleep, or sleeping too much/i)
			).toBeInTheDocument();
		});

		// Check that all options are rendered for the first question
		const notAtAllButtons = screen.getAllByText('Not at all');
		expect(notAtAllButtons.length).toBeGreaterThan(0);

		const severalDaysButtons = screen.getAllByText('Several days');
		expect(severalDaysButtons.length).toBeGreaterThan(0);
	});

	it('displays progress indicator', async () => {
		mockInvoke.mockResolvedValue(mockQuestions);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			// Progress should show 0/3 initially
			expect(screen.getByText(/Progress: 0\/3/i)).toBeInTheDocument();
		});
	});

	it('renders back navigation link', async () => {
		mockInvoke.mockResolvedValue(mockQuestions);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			const backLink = screen.getByText(/Back to Assessments/i);
			expect(backLink).toBeInTheDocument();
			expect(backLink.closest('a')).toHaveAttribute('href', '/assessments');
		});
	});

	it('renders optional notes textarea', async () => {
		mockInvoke.mockResolvedValue(mockQuestions);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			const notesTextarea = screen.getByPlaceholderText(/Any additional thoughts or context/i);
			expect(notesTextarea).toBeInTheDocument();
			expect(notesTextarea.tagName).toBe('TEXTAREA');
		});
	});

	it('renders submit button initially disabled', async () => {
		mockInvoke.mockResolvedValue(mockQuestions);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			const submitButton = screen.getByRole('button', { name: /Submit Assessment/i });
			expect(submitButton).toBeInTheDocument();
			expect(submitButton).toBeDisabled();
		});
	});

	it('displays error when API call fails', async () => {
		mockInvoke.mockRejectedValue(new Error('Failed to fetch questions'));

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			expect(screen.getByText(/Error:/i)).toBeInTheDocument();
			expect(screen.getByText(/Failed to fetch questions/i)).toBeInTheDocument();
		});
	});

	it('renders progress bar with correct aria attributes', async () => {
		mockInvoke.mockResolvedValue(mockQuestions);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			const progressBar = screen.getByRole('progressbar');
			expect(progressBar).toBeInTheDocument();
			expect(progressBar).toHaveAttribute('aria-valuenow', '0');
			expect(progressBar).toHaveAttribute('aria-valuemin', '0');
			expect(progressBar).toHaveAttribute('aria-valuemax', '100');
			expect(progressBar).toHaveAttribute('aria-label', 'Assessment completion progress');
		});
	});

	it('renders radio button groups with proper aria labels', async () => {
		mockInvoke.mockResolvedValue(mockQuestions);

		render(AssessmentForm, { props: { assessmentCode: 'PHQ9' } });

		await waitFor(() => {
			const radioGroups = screen.getAllByRole('radiogroup');
			expect(radioGroups.length).toBe(3); // One for each question

			// Check that radio groups have proper aria-labelledby
			radioGroups.forEach((group, index) => {
				expect(group).toHaveAttribute('aria-labelledby', `question-${index}-label`);
			});
		});
	});
});
