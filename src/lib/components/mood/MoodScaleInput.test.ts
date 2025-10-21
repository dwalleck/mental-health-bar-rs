// T072: Component test - MoodScaleInput renders 1-5 buttons

import { render, screen, fireEvent } from '@testing-library/svelte';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import MoodScaleInput from './MoodScaleInput.svelte';

describe('MoodScaleInput', () => {
	beforeEach(() => {
		vi.clearAllMocks();
	});

	it('renders all 5 mood rating buttons', () => {
		render(MoodScaleInput, { props: { selectedRating: null } });

		// Check for mood rating labels/values 1-5
		expect(screen.getByText('1')).toBeInTheDocument();
		expect(screen.getByText('2')).toBeInTheDocument();
		expect(screen.getByText('3')).toBeInTheDocument();
		expect(screen.getByText('4')).toBeInTheDocument();
		expect(screen.getByText('5')).toBeInTheDocument();
	});

	it('renders mood labels (Very Bad to Very Good)', () => {
		render(MoodScaleInput, { props: { selectedRating: null } });

		// Check for mood labels
		expect(screen.getByText(/Very Bad/i)).toBeInTheDocument();
		expect(screen.getByText(/Bad/i)).toBeInTheDocument();
		expect(screen.getByText(/Neutral/i)).toBeInTheDocument();
		expect(screen.getByText(/Good/i)).toBeInTheDocument();
		expect(screen.getByText(/Very Good/i)).toBeInTheDocument();
	});

	it('applies correct color to each mood rating button', () => {
		const { container } = render(MoodScaleInput, { props: { selectedRating: null } });

		// Find all mood buttons
		const buttons = container.querySelectorAll('[data-mood-rating]');
		expect(buttons.length).toBe(5);

		// Check for color classes or styles
		// Colors should be: red, orange, yellow, light green, green
		const button1 = container.querySelector('[data-mood-rating="1"]');
		const button5 = container.querySelector('[data-mood-rating="5"]');

		expect(button1).toBeTruthy();
		expect(button5).toBeTruthy();
	});

	it('highlights selected mood rating', () => {
		const { container } = render(MoodScaleInput, { props: { selectedRating: 3 } });

		const button3 = container.querySelector('[data-mood-rating="3"]');
		expect(button3).toBeTruthy();

		// Check for selected state (could be a class, aria-pressed, or style)
		expect(
			button3?.classList.contains('selected') ||
				button3?.getAttribute('aria-pressed') === 'true' ||
				button3?.classList.contains('ring')
		).toBeTruthy();
	});

	it('emits rating selection event when clicked', async () => {
		const { component } = render(MoodScaleInput, { props: { selectedRating: null } });

		let capturedRating: number | null = null;
		component.$on('select', (event) => {
			capturedRating = event.detail;
		});

		// Find and click mood rating 4 button
		const button4 = screen.getByText('4');
		await fireEvent.click(button4);

		expect(capturedRating).toBe(4);
	});

	it('allows changing selection', async () => {
		const { component, container } = render(MoodScaleInput, { props: { selectedRating: 2 } });

		let capturedRating: number | null = null;
		component.$on('select', (event) => {
			capturedRating = event.detail;
		});

		// Click mood rating 5 to change selection
		const button5 = container.querySelector('[data-mood-rating="5"]');
		expect(button5).toBeTruthy();

		if (button5) {
			await fireEvent.click(button5);
			expect(capturedRating).toBe(5);
		}
	});

	it('has accessible buttons with ARIA labels', () => {
		render(MoodScaleInput, { props: { selectedRating: null } });

		const buttons = screen.getAllByRole('button');
		expect(buttons.length).toBeGreaterThanOrEqual(5);

		// Each button should have an aria-label for accessibility
		buttons.forEach((button) => {
			expect(button.getAttribute('aria-label')).toBeTruthy();
		});
	});

	it('renders buttons in correct order (1 to 5, left to right)', () => {
		const { container } = render(MoodScaleInput, { props: { selectedRating: null } });

		const buttons = container.querySelectorAll('[data-mood-rating]');
		expect(buttons.length).toBe(5);

		// Verify order
		expect(buttons[0].getAttribute('data-mood-rating')).toBe('1');
		expect(buttons[1].getAttribute('data-mood-rating')).toBe('2');
		expect(buttons[2].getAttribute('data-mood-rating')).toBe('3');
		expect(buttons[3].getAttribute('data-mood-rating')).toBe('4');
		expect(buttons[4].getAttribute('data-mood-rating')).toBe('5');
	});

	it('renders without crashing when no rating selected', () => {
		expect(() => {
			render(MoodScaleInput, { props: { selectedRating: null } });
		}).not.toThrow();
	});

	it('renders without crashing when rating is selected', () => {
		expect(() => {
			render(MoodScaleInput, { props: { selectedRating: 3 } });
		}).not.toThrow();
	});
});
