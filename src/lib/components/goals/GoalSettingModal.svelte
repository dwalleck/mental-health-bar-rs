<script lang="ts">
	// T116: GoalSettingModal - Create/edit activity goals
	import { commands } from '$lib/bindings'
	import type { ActivityGoal, Activity, ActivityGroup } from '$lib/bindings'
	import { GOAL_TYPES, type GoalType } from '$lib/constants/activities'
	import Modal from '$lib/components/ui/Modal.svelte'
	import { displayError, displaySuccess } from '$lib/utils/errors'

	interface Props {
		open: boolean
		activity?: Activity
		group?: ActivityGroup
		existingGoal?: ActivityGoal
		onSuccess: () => void
		onCancel: () => void
	}

	let { open = $bindable(), activity, group, existingGoal, onSuccess, onCancel }: Props = $props()

	// Validate that either activity or group is provided
	$effect(() => {
		if (open && !activity && !group) {
			console.error('GoalSettingModal: Either activity or group must be provided')
			onCancel()
		}
	})

	// Determine if editing or creating
	let isEditMode = $derived(existingGoal !== undefined)

	// Form state
	let goalType = $state<GoalType>(
		(existingGoal?.goal_type as GoalType) || GOAL_TYPES.DAYS_PER_PERIOD
	)
	let targetValue = $state(existingGoal?.target_value ?? 3)
	let periodDays = $state(existingGoal?.period_days ?? 7)
	let customPeriod = $state('')
	let isSubmitting = $state(false)
	let errors = $state<Record<string, string>>({})

	// Period presets
	const periodPresets = [
		{ label: '7 days (1 week)', value: 7 },
		{ label: '14 days (2 weeks)', value: 14 },
		{ label: '30 days (1 month)', value: 30 },
		{ label: 'Custom', value: -1 },
	]

	// Reset form when modal opens
	$effect(() => {
		if (open) {
			goalType = (existingGoal?.goal_type as GoalType) || GOAL_TYPES.DAYS_PER_PERIOD
			targetValue = existingGoal?.target_value ?? 3
			periodDays = existingGoal?.period_days ?? 7
			customPeriod = ''
			errors = {}
		}
	})

	function validateForm(): boolean {
		const newErrors: Record<string, string> = {}

		// Calculate effective period (handles custom period case)
		const effectivePeriod = periodDays === -1 ? parseInt(customPeriod) || 0 : periodDays

		// Validate target value
		if (!targetValue || targetValue <= 0) {
			newErrors.targetValue = 'Target value must be a positive number'
		}

		// FIXED: Use effectivePeriod instead of periodDays to handle custom period correctly
		if (goalType === GOAL_TYPES.DAYS_PER_PERIOD) {
			if (effectivePeriod <= 0) {
				newErrors.periodDays = 'Custom period must be a positive number'
			} else if (targetValue > effectivePeriod) {
				newErrors.targetValue = `Cannot exceed ${effectivePeriod} days in the period`
			}
		}

		if (goalType === GOAL_TYPES.PERCENT_IMPROVEMENT && targetValue > 1000) {
			newErrors.targetValue = 'Percentage cannot exceed 1000%'
		}

		// Validate period
		if (periodDays === -1) {
			const custom = parseInt(customPeriod)
			if (!custom || custom <= 0) {
				newErrors.periodDays = 'Custom period must be a positive number'
			} else if (custom > 365) {
				newErrors.periodDays = 'Period cannot exceed 365 days'
			}
		}

		errors = newErrors
		return Object.keys(newErrors).length === 0
	}

	async function handleSubmit() {
		if (!validateForm()) return

		try {
			isSubmitting = true

			// Use custom period if selected
			const finalPeriod = periodDays === -1 ? parseInt(customPeriod) : periodDays

			if (isEditMode && existingGoal) {
				// Update existing goal
				const result = await commands.updateActivityGoal(existingGoal.id, targetValue, finalPeriod)

				if (result.status === 'error') {
					throw new Error(result.error.message)
				}

				displaySuccess('Goal updated successfully')
			} else {
				// Create new goal
				const result = await commands.setActivityGoal({
					activity_id: activity?.id ?? null,
					group_id: group?.id ?? null,
					goal_type: goalType,
					target_value: targetValue,
					period_days: finalPeriod,
				})

				if (result.status === 'error') {
					throw new Error(result.error.message)
				}

				displaySuccess('Goal created successfully')
			}

			open = false
			onSuccess()
		} catch (error) {
			displayError(error)
		} finally {
			isSubmitting = false
		}
	}

	// Helper text for goal types
	let goalTypeDescription = $derived(
		goalType === GOAL_TYPES.DAYS_PER_PERIOD
			? `Track how many days you complete this activity within a ${periodDays === -1 ? customPeriod || '?' : periodDays}-day period.`
			: `Track percentage improvement compared to your ${periodDays === -1 ? customPeriod || '?' : periodDays}-day baseline average.`
	)

	// Target label based on goal type
	let targetLabel = $derived(
		goalType === GOAL_TYPES.DAYS_PER_PERIOD ? 'Target Days' : 'Target Percentage'
	)
	let targetPlaceholder = $derived(goalType === GOAL_TYPES.DAYS_PER_PERIOD ? 'e.g., 3' : 'e.g., 20')
</script>

<Modal
	bind:open
	title={isEditMode ? 'Edit Goal' : 'Set New Goal'}
	description={activity
		? `Set a goal for ${activity.name}`
		: group
			? `Set a goal for ${group.name} group`
			: 'Set an activity goal'}
	size="md"
	actions={[
		{
			label: 'Cancel',
			variant: 'secondary',
			onClick: onCancel,
			disabled: isSubmitting,
		},
		{
			label: isEditMode ? 'Update Goal' : 'Create Goal',
			variant: 'primary',
			onClick: handleSubmit,
			disabled: isSubmitting,
			loading: isSubmitting,
		},
	]}
>
	<form
		class="space-y-6"
		onsubmit={(e) => {
			e.preventDefault()
			handleSubmit()
		}}
	>
		<!-- Goal Type Selector (Task 3.19) -->
		<div>
			<label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
				Goal Type <span class="text-red-500">*</span>
			</label>
			<div class="space-y-3">
				<label class="flex items-start gap-3 cursor-pointer">
					<input
						type="radio"
						bind:group={goalType}
						value={GOAL_TYPES.DAYS_PER_PERIOD}
						disabled={isEditMode || isSubmitting}
						class="mt-1"
					/>
					<div class="flex-1">
						<div class="font-medium text-gray-900 dark:text-white">Days per Period</div>
						<div class="text-sm text-gray-600 dark:text-gray-400">
							Track how many days you complete this activity (e.g., "3 days per week")
						</div>
					</div>
				</label>

				<label class="flex items-start gap-3 cursor-pointer">
					<input
						type="radio"
						bind:group={goalType}
						value={GOAL_TYPES.PERCENT_IMPROVEMENT}
						disabled={isEditMode || isSubmitting}
						class="mt-1"
					/>
					<div class="flex-1">
						<div class="font-medium text-gray-900 dark:text-white">Percent Improvement</div>
						<div class="text-sm text-gray-600 dark:text-gray-400">
							Track percentage improvement over your baseline (e.g., "20% more than usual")
						</div>
					</div>
				</label>
			</div>
			{#if isEditMode}
				<p class="text-xs text-gray-500 dark:text-gray-400 mt-2">
					Goal type cannot be changed after creation
				</p>
			{/if}
		</div>

		<!-- Dynamic Description -->
		<div
			class="bg-blue-50 dark:bg-blue-900/20 border border-blue-200 dark:border-blue-800 rounded-lg p-3"
		>
			<p class="text-sm text-blue-900 dark:text-blue-200">{goalTypeDescription}</p>
		</div>

		<!-- Target Value Input (Task 3.20) -->
		<div>
			<label
				for="target-value"
				class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1"
			>
				{targetLabel} <span class="text-red-500">*</span>
			</label>
			<input
				id="target-value"
				type="number"
				min="1"
				max={goalType === GOAL_TYPES.PERCENT_IMPROVEMENT ? 1000 : undefined}
				bind:value={targetValue}
				placeholder={targetPlaceholder}
				disabled={isSubmitting}
				class="w-full px-4 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500
					{errors.targetValue ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}
					dark:bg-gray-700 dark:text-white"
			/>
			{#if errors.targetValue}
				<p class="mt-1 text-sm text-red-500">{errors.targetValue}</p>
			{:else}
				<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
					{#if goalType === GOAL_TYPES.DAYS_PER_PERIOD}
						Number of days to complete this activity
					{:else}
						Percentage improvement (e.g., 20 for 20%)
					{/if}
				</p>
			{/if}
		</div>

		<!-- Period Selector (Task 3.21) -->
		<div>
			<label for="period" class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
				Time Period <span class="text-red-500">*</span>
			</label>
			<select
				id="period"
				bind:value={periodDays}
				disabled={isSubmitting}
				class="w-full px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-lg
					focus:outline-hidden focus:ring-2 focus:ring-blue-500 dark:bg-gray-700 dark:text-white"
			>
				{#each periodPresets as preset (preset.value)}
					<option value={preset.value}>{preset.label}</option>
				{/each}
			</select>

			{#if periodDays === -1}
				<div class="mt-2">
					<input
						type="number"
						min="1"
						max="365"
						bind:value={customPeriod}
						placeholder="Enter custom period (days)"
						disabled={isSubmitting}
						class="w-full px-4 py-2 border rounded-lg focus:outline-hidden focus:ring-2 focus:ring-blue-500
							{errors.periodDays ? 'border-red-500' : 'border-gray-300 dark:border-gray-600'}
							dark:bg-gray-700 dark:text-white"
					/>
					{#if errors.periodDays}
						<p class="mt-1 text-sm text-red-500">{errors.periodDays}</p>
					{/if}
				</div>
			{/if}

			<p class="mt-1 text-xs text-gray-500 dark:text-gray-400">
				{#if goalType === GOAL_TYPES.DAYS_PER_PERIOD}
					Rolling window for tracking days
				{:else}
					Baseline period for comparison
				{/if}
			</p>
		</div>
	</form>
</Modal>
