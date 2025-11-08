<script lang="ts">
	import SidebarLayout from '$lib/components/ui/SidebarLayout.svelte'
	import FormLayout from '$lib/components/ui/FormLayout.svelte'
	import DataTable from '$lib/components/ui/DataTable.svelte'
	import Modal from '$lib/components/ui/Modal.svelte'
	import Combobox from '$lib/components/ui/Combobox.svelte'
	import { displaySuccess, displayInfo, displayWarning } from '$lib/utils/errors'

	// Type definitions
	interface AssessmentTableItem {
		id: number
		date: string
		type: string
		score: number
		severity: string
		status: string
	}

	// Navigation items for sidebar
	const navItems = [
		{ name: 'Dashboard', href: '/', icon: 'home' },
		{ name: 'Assessments', href: '/assessments', icon: 'clipboard', badge: 2 },
		{ name: 'Mood', href: '/mood', icon: 'emoticon' },
		{ name: 'Charts', href: '/charts', icon: 'chart' },
		{ name: 'UI Showcase', href: '/ui-showcase', icon: 'cog' },
		{ name: 'Settings', href: '/settings', icon: 'cog' },
	]

	// Sample data for table
	const tableData = [
		{
			id: 1,
			date: '2025-10-28',
			type: 'PHQ-9',
			score: 12,
			severity: 'Moderate',
			status: 'Completed',
		},
		{
			id: 2,
			date: '2025-10-27',
			type: 'GAD-7',
			score: 8,
			severity: 'Mild',
			status: 'Completed',
		},
		{
			id: 3,
			date: '2025-10-26',
			type: 'CES-D',
			score: 18,
			severity: 'Moderate',
			status: 'Completed',
		},
		{
			id: 4,
			date: '2025-10-25',
			type: 'OASIS',
			score: 6,
			severity: 'Mild',
			status: 'Completed',
		},
		{
			id: 5,
			date: '2025-10-24',
			type: 'PHQ-9',
			score: 15,
			severity: 'Moderate',
			status: 'Completed',
		},
	]

	const tableColumns = [
		{ key: 'date', label: 'Date', sortable: true },
		{ key: 'type', label: 'Assessment Type', sortable: true },
		{
			key: 'score',
			label: 'Score',
			sortable: true,
			align: 'center' as const,
			render: (item: AssessmentTableItem) => `<span class="font-semibold">${item.score}</span>`,
		},
		{
			key: 'severity',
			label: 'Severity',
			render: (item: AssessmentTableItem) => {
				const colors: Record<string, string> = {
					Mild: 'text-yellow-600 bg-yellow-100 dark:text-yellow-400 dark:bg-yellow-900',
					Moderate: 'text-orange-600 bg-orange-100 dark:text-orange-400 dark:bg-orange-900',
					Severe: 'text-red-600 bg-red-100 dark:text-red-400 dark:bg-red-900',
				}
				return `<span class="inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium ${
					colors[item.severity] || ''
				}">${item.severity}</span>`
			},
		},
		{
			key: 'status',
			label: 'Status',
			render: (item: AssessmentTableItem) =>
				`<span class="inline-flex items-center rounded-full bg-green-100 dark:bg-green-900 px-2.5 py-0.5 text-xs font-medium text-green-800 dark:text-green-200">${item.status}</span>`,
		},
	]

	const tableActions = [
		{
			label: 'View',
			onClick: (item: AssessmentTableItem) => displayInfo(`Viewing assessment ${item.id}`),
		},
		{
			label: 'Delete',
			variant: 'danger' as const,
			onClick: (item: AssessmentTableItem) => displayWarning(`Delete assessment ${item.id}?`),
		},
	]

	// Modal states
	let showModal = $state(false)
	let showDeleteModal = $state(false)

	// Form states
	let formLoading = $state(false)
	let selectedAssessment = $state<string | number | null>(null)
	let selectedActivities = $state<(string | number)[]>([])
	let moodRating = $state(3)
	let notes = $state('')

	// Combobox options
	const assessmentOptions = [
		{
			value: 'phq9',
			label: 'PHQ-9 Depression Scale',
			description: '9 questions about depression symptoms',
		},
		{
			value: 'gad7',
			label: 'GAD-7 Anxiety Scale',
			description: '7 questions about anxiety symptoms',
		},
		{
			value: 'cesd',
			label: 'CES-D Depression Scale',
			description: '20 questions comprehensive assessment',
		},
		{
			value: 'oasis',
			label: 'OASIS Anxiety Scale',
			description: '5 questions quick anxiety screening',
		},
	]

	const activityOptions = [
		{ value: 1, label: 'Exercise', icon: '🏃' },
		{ value: 2, label: 'Reading', icon: '📚' },
		{ value: 3, label: 'Meditation', icon: '🧘' },
		{ value: 4, label: 'Socializing', icon: '👥' },
		{ value: 5, label: 'Work', icon: '💼' },
		{ value: 6, label: 'Relaxing', icon: '😌' },
	]

	// Form submission
	async function handleFormSubmit() {
		formLoading = true
		// Simulate API call
		await new Promise((resolve) => setTimeout(resolve, 2000))
		formLoading = false
		displaySuccess('Assessment scheduled successfully!')
	}
</script>

<SidebarLayout {navItems} userProfile={{ name: 'John Doe', email: 'john@example.com' }}>
	<div class="space-y-8">
		<div>
			<h1 class="text-3xl font-bold text-gray-900 dark:text-white">
				Tailwind UI Components Showcase
			</h1>
			<p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
				Modern UI components integrated with your mental health tracking app
			</p>
		</div>

		<!-- Enhanced Form Layout -->
		<div class="bg-white dark:bg-gray-800 shadow-sm rounded-lg p-6">
			<h2 class="text-xl font-semibold mb-6">Enhanced Form Layout</h2>
			<FormLayout
				title="Schedule Assessment"
				description="Configure when you want to receive assessment reminders"
				sections={[
					{
						title: 'Assessment Details',
						description: 'Choose your assessment type and frequency',
					},
					{
						title: 'Additional Information',
						description: 'Add any notes or context',
					},
				]}
				onSubmit={handleFormSubmit}
				loading={formLoading}
			>
				{#snippet section_0()}
					<div class="form-col-span-4">
						<Combobox
							options={assessmentOptions}
							bind:value={selectedAssessment}
							label="Assessment Type"
							description="Select the assessment you want to schedule"
							placeholder="Choose an assessment"
							required
						/>
					</div>

					<div class="form-col-span-2">
						<label for="frequency" class="form-label">Frequency</label>
						<select id="frequency" class="form-select">
							<option>Daily</option>
							<option>Weekly</option>
							<option>Bi-weekly</option>
							<option>Monthly</option>
						</select>
					</div>

					<div class="form-col-span-2">
						<label for="time" class="form-label">Time</label>
						<input type="time" id="time" value="09:00" class="form-input" />
					</div>
				{/snippet}

				{#snippet section_1()}
					<div class="form-col-span-full">
						<label for="notes" class="form-label">Notes</label>
						<textarea
							id="notes"
							bind:value={notes}
							rows="4"
							class="form-textarea"
							placeholder="Add any additional notes or reminders..."
						></textarea>
						<p class="form-description">These notes will be included with your reminder</p>
					</div>
				{/snippet}
			</FormLayout>
		</div>

		<!-- Professional Data Table -->
		<div class="bg-white dark:bg-gray-800 shadow-sm rounded-lg p-6">
			<h2 class="text-xl font-semibold mb-6">Professional Data Table</h2>
			<DataTable
				columns={tableColumns}
				data={tableData}
				actions={tableActions}
				selectable={true}
				pagination={true}
				pageSize={3}
			/>
		</div>

		<!-- Enhanced Combobox -->
		<div class="bg-white dark:bg-gray-800 shadow-sm rounded-lg p-6">
			<h2 class="text-xl font-semibold mb-6">Enhanced Combobox Components</h2>
			<div class="grid grid-cols-1 md:grid-cols-2 gap-6">
				<Combobox
					options={assessmentOptions}
					bind:value={selectedAssessment}
					label="Single Select with Search"
					description="Choose one assessment type"
					placeholder="Search assessments..."
				/>

				<Combobox
					options={activityOptions}
					bind:value={selectedActivities}
					label="Multi-Select Activities"
					description="Select multiple activities"
					placeholder="Choose activities..."
					multiple={true}
				/>
			</div>
		</div>

		<!-- Modal Examples -->
		<div class="bg-white dark:bg-gray-800 shadow-sm rounded-lg p-6">
			<h2 class="text-xl font-semibold mb-6">Modal Components</h2>
			<div class="flex gap-4">
				<button
					class="inline-flex justify-center rounded-md bg-blue-600 px-4 py-2 text-sm font-semibold text-white shadow-xs hover:bg-blue-500"
					onclick={() => (showModal = true)}
				>
					Open Standard Modal
				</button>

				<button
					class="inline-flex justify-center rounded-md bg-red-600 px-4 py-2 text-sm font-semibold text-white shadow-xs hover:bg-red-500"
					onclick={() => (showDeleteModal = true)}
				>
					Open Delete Confirmation
				</button>
			</div>
		</div>

		<!-- Mood Scale Example -->
		<div class="bg-white dark:bg-gray-800 shadow-sm rounded-lg p-6">
			<h2 class="text-xl font-semibold mb-6">Mood Check-in Form</h2>
			<FormLayout
				title="How are you feeling?"
				description="Track your mood and activities"
				onSubmit={() => displaySuccess('Mood logged successfully!')}
				submitLabel="Log Mood"
				showCancel={false}
			>
				<div class="form-col-span-full">
					<label class="form-label mb-3">Mood Rating</label>
					<div class="flex gap-2">
						{#each [1, 2, 3, 4, 5] as rating (rating)}
							<button
								type="button"
								class="flex-1 py-3 px-4 rounded-lg border-2 transition-colors {moodRating === rating
									? 'border-blue-600 bg-blue-50 dark:bg-blue-900'
									: 'border-gray-300 dark:border-gray-700 hover:border-gray-400'}"
								onclick={() => (moodRating = rating)}
							>
								<div class="text-2xl mb-1">
									{rating === 1
										? '😔'
										: rating === 2
											? '😟'
											: rating === 3
												? '😐'
												: rating === 4
													? '🙂'
													: '😄'}
								</div>
								<div class="text-xs text-gray-600 dark:text-gray-400">
									{rating === 1
										? 'Very Bad'
										: rating === 2
											? 'Bad'
											: rating === 3
												? 'Neutral'
												: rating === 4
													? 'Good'
													: 'Very Good'}
								</div>
							</button>
						{/each}
					</div>
				</div>

				<div class="form-col-span-full">
					<Combobox
						options={activityOptions}
						bind:value={selectedActivities}
						label="What have you been doing?"
						description="Select all activities that apply"
						placeholder="Choose activities..."
						multiple={true}
					/>
				</div>
			</FormLayout>
		</div>
	</div>
</SidebarLayout>

<!-- Standard Modal -->
<Modal
	bind:open={showModal}
	title="Schedule Assessment"
	description="Would you like to schedule a PHQ-9 assessment for tomorrow at 9:00 AM?"
	size="md"
	actions={[
		{
			label: 'Cancel',
			variant: 'secondary',
			onClick: () => {
				showModal = false
			},
		},
		{
			label: 'Schedule',
			variant: 'primary',
			onClick: async () => {
				await new Promise((r) => setTimeout(r, 1000))
				showModal = false
				displaySuccess('Assessment scheduled!')
			},
		},
	]}
>
	<div class="space-y-4">
		<div class="flex items-center justify-between py-3 px-4 bg-gray-50 dark:bg-gray-800 rounded-lg">
			<div>
				<p class="font-medium text-gray-900 dark:text-white">PHQ-9 Depression Scale</p>
				<p class="text-sm text-gray-500 dark:text-gray-400">9 questions • ~5 minutes</p>
			</div>
			<div class="text-right">
				<p class="text-sm text-gray-500 dark:text-gray-400">Tomorrow</p>
				<p class="font-medium text-gray-900 dark:text-white">9:00 AM</p>
			</div>
		</div>
	</div>
</Modal>

<!-- Delete Confirmation Modal -->
<Modal
	bind:open={showDeleteModal}
	title="Delete Assessment"
	description="Are you sure you want to delete this assessment? This action cannot be undone."
	size="sm"
	actions={[
		{
			label: 'Cancel',
			variant: 'secondary',
			onClick: () => {
				showDeleteModal = false
			},
		},
		{
			label: 'Delete',
			variant: 'danger',
			onClick: () => {
				showDeleteModal = false
				displayWarning('Assessment deleted')
			},
		},
	]}
>
	{#snippet icon()}
		<div>
			<svg
				class="h-6 w-6 text-red-600"
				fill="none"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M12 9v3.75m-9.303 3.376c-.866 1.5.217 3.374 1.948 3.374h14.71c1.73 0 2.813-1.874 1.948-3.374L13.949 3.378c-.866-1.5-3.032-1.5-3.898 0L2.697 16.126zM12 15.75h.007v.008H12v-.008z"
				/>
			</svg>
		</div>
	{/snippet}
</Modal>
