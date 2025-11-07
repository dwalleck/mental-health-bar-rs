<script lang="ts">
	import Card from '$lib/components/ui/Card.svelte'
	import Button from '$lib/components/ui/Button.svelte'
	import { displaySuccess, displayInfo } from '$lib/utils/errors'
</script>

<div class="space-y-8">
	<div>
		<h1 class="text-3xl font-bold text-gray-900 dark:text-white">✨ New Tailwind UI Components</h1>
		<p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
			Professional UI components ready to integrate into your mental health tracker
		</p>
	</div>

	<!-- Component Gallery -->
	<div class="grid gap-6 md:grid-cols-2">
		<!-- Sidebar Layout Preview -->
		<Card title="🎯 Modern Sidebar Navigation">
			<div class="space-y-4">
				<p class="text-sm text-gray-600 dark:text-gray-400">
					A responsive sidebar with collapsible mobile menu, user profile section, and badge
					support.
				</p>
				<div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
					<div class="flex gap-2">
						<div class="w-16 h-48 bg-gray-100 dark:bg-gray-800 rounded-sm"></div>
						<div class="flex-1 space-y-2">
							<div class="h-8 bg-gray-50 dark:bg-gray-900 rounded-sm"></div>
							<div class="h-32 bg-gray-50 dark:bg-gray-900 rounded-sm"></div>
						</div>
					</div>
				</div>
				<p class="text-xs text-gray-500">
					<strong>Location:</strong> <code>/lib/components/ui/SidebarLayout.svelte</code>
				</p>
			</div>
		</Card>

		<!-- Form Layout Preview -->
		<Card title="📝 Enhanced Form Layouts">
			<div class="space-y-4">
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Professional forms with sections, proper spacing, loading states, and validation.
				</p>
				<div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
					<div class="space-y-3">
						<div class="h-6 w-32 bg-gray-200 dark:bg-gray-700 rounded-sm"></div>
						<div class="h-10 bg-gray-50 dark:bg-gray-900 rounded-sm"></div>
						<div class="h-10 bg-gray-50 dark:bg-gray-900 rounded-sm"></div>
						<div class="flex gap-2 justify-end">
							<div class="h-8 w-20 bg-gray-200 dark:bg-gray-700 rounded-sm"></div>
							<div class="h-8 w-20 bg-blue-600 rounded-sm"></div>
						</div>
					</div>
				</div>
				<p class="text-xs text-gray-500">
					<strong>Location:</strong> <code>/lib/components/ui/FormLayout.svelte</code>
				</p>
			</div>
		</Card>

		<!-- Data Table Preview -->
		<Card title="📊 Professional Data Tables">
			<div class="space-y-4">
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Sortable columns, pagination, row selection, and action buttons.
				</p>
				<div class="border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden">
					<table class="min-w-full divide-y divide-gray-200 dark:divide-gray-700">
						<thead class="bg-gray-50 dark:bg-gray-800">
							<tr>
								<th class="px-3 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400"
									>Date</th
								>
								<th class="px-3 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400"
									>Type</th
								>
								<th class="px-3 py-2 text-left text-xs font-medium text-gray-500 dark:text-gray-400"
									>Score</th
								>
							</tr>
						</thead>
						<tbody class="bg-white dark:bg-gray-900 divide-y divide-gray-200 dark:divide-gray-700">
							<tr>
								<td class="px-3 py-2 text-xs">2025-10-28</td>
								<td class="px-3 py-2 text-xs">PHQ-9</td>
								<td class="px-3 py-2 text-xs">12</td>
							</tr>
							<tr class="bg-gray-50 dark:bg-gray-800/50">
								<td class="px-3 py-2 text-xs">2025-10-27</td>
								<td class="px-3 py-2 text-xs">GAD-7</td>
								<td class="px-3 py-2 text-xs">8</td>
							</tr>
						</tbody>
					</table>
				</div>
				<p class="text-xs text-gray-500">
					<strong>Location:</strong> <code>/lib/components/ui/DataTable.svelte</code>
				</p>
			</div>
		</Card>

		<!-- Modal Preview -->
		<Card title="💬 Modal Dialogs">
			<div class="space-y-4">
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Multiple sizes, backdrop blur, focus trap, and smooth animations.
				</p>
				<div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
					<div class="relative">
						<div class="bg-gray-100/50 dark:bg-gray-800/50 rounded-lg p-4">
							<div class="bg-white dark:bg-gray-900 rounded-lg shadow-lg p-4 space-y-2">
								<div class="h-4 w-32 bg-gray-800 dark:bg-gray-200 rounded-sm"></div>
								<div class="h-3 w-full bg-gray-200 dark:bg-gray-700 rounded-sm"></div>
								<div class="flex gap-2 justify-end mt-4">
									<div class="h-6 w-16 bg-gray-300 dark:bg-gray-700 rounded-sm text-xs"></div>
									<div class="h-6 w-16 bg-blue-600 rounded-sm"></div>
								</div>
							</div>
						</div>
					</div>
				</div>
				<Button variant="primary" onclick={() => displayInfo('Modal component ready to use!')}>
					View Example
				</Button>
				<p class="text-xs text-gray-500">
					<strong>Location:</strong> <code>/lib/components/ui/Modal.svelte</code>
				</p>
			</div>
		</Card>

		<!-- Combobox Preview -->
		<Card title="🔍 Enhanced Combobox">
			<div class="space-y-4">
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Search functionality, multi-select, keyboard navigation, and custom rendering.
				</p>
				<div class="border border-gray-200 dark:border-gray-700 rounded-lg p-4">
					<div class="space-y-2">
						<div
							class="border border-gray-300 dark:border-gray-700 rounded-md p-2 flex justify-between"
						>
							<span class="text-sm text-gray-500">Select an option...</span>
							<svg class="h-4 w-4 text-gray-400" fill="currentColor" viewBox="0 0 20 20">
								<path
									fill-rule="evenodd"
									d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
									clip-rule="evenodd"
								/>
							</svg>
						</div>
					</div>
				</div>
				<p class="text-xs text-gray-500">
					<strong>Location:</strong> <code>/lib/components/ui/Combobox.svelte</code>
				</p>
			</div>
		</Card>

		<!-- Announcement Banner Preview -->
		<Card title="📢 Announcement Banner">
			<div class="space-y-4">
				<p class="text-sm text-gray-600 dark:text-gray-400">
					Highlight new features or important messages with dismissible banners.
				</p>
				<div
					class="border border-blue-200 dark:border-blue-800 bg-blue-50 dark:bg-blue-900/20 rounded-lg p-3"
				>
					<div class="flex items-start">
						<div class="text-blue-600 dark:text-blue-400">ℹ️</div>
						<div class="ml-2 flex-1">
							<p class="text-sm font-medium text-blue-800 dark:text-blue-200">
								New feature available!
							</p>
							<p class="text-xs text-blue-700 dark:text-blue-300 mt-1">
								Check out the latest updates
							</p>
						</div>
						<button class="text-blue-600 dark:text-blue-400">✕</button>
					</div>
				</div>
				<p class="text-xs text-gray-500">
					<strong>Location:</strong> <code>/lib/components/ui/AnnouncementBanner.svelte</code>
				</p>
			</div>
		</Card>
	</div>

	<!-- Integration Guide -->
	<Card title="🚀 How to Use These Components">
		<div class="space-y-6">
			<div>
				<h3 class="font-semibold text-gray-900 dark:text-white mb-2">1. Navigation Upgrade</h3>
				<p class="text-sm text-gray-600 dark:text-gray-400 mb-2">
					Replace the current sidebar with the modern SidebarLayout for a professional look.
				</p>
				<pre
					class="bg-gray-100 dark:bg-gray-800 p-3 rounded-sm text-xs overflow-x-auto">{`<SidebarLayout navItems={items}>
  <slot />
</SidebarLayout>`}</pre>
			</div>

			<div>
				<h3 class="font-semibold text-gray-900 dark:text-white mb-2">2. Form Enhancement</h3>
				<p class="text-sm text-gray-600 dark:text-gray-400 mb-2">
					Use FormLayout for consistent, professional forms with built-in validation.
				</p>
				<pre
					class="bg-gray-100 dark:bg-gray-800 p-3 rounded-sm text-xs overflow-x-auto">{`<FormLayout
  title="Assessment"
  sections={[...]}
  onSubmit={handle}>
  <!-- fields -->
</FormLayout>`}</pre>
			</div>

			<div>
				<h3 class="font-semibold text-gray-900 dark:text-white mb-2">3. Data Display</h3>
				<p class="text-sm text-gray-600 dark:text-gray-400 mb-2">
					Replace basic tables with DataTable for sorting, pagination, and actions.
				</p>
				<pre
					class="bg-gray-100 dark:bg-gray-800 p-3 rounded-sm text-xs overflow-x-auto">{`<DataTable
  columns={cols}
  data={items}
  actions={[...]}
  pagination={true} />`}</pre>
			</div>

			<div>
				<h3 class="font-semibold text-gray-900 dark:text-white mb-2">4. User Interactions</h3>
				<p class="text-sm text-gray-600 dark:text-gray-400 mb-2">
					Replace confirm() dialogs with Modal components for better UX.
				</p>
				<pre class="bg-gray-100 dark:bg-gray-800 p-3 rounded-sm text-xs overflow-x-auto">{`<Modal
  open={show}
  title="Confirm"
  actions={[...]} />`}</pre>
			</div>

			<div class="flex gap-4 pt-4">
				<Button variant="primary" onclick={() => displaySuccess('Ready to upgrade your UI!')}>
					Start Integration
				</Button>
				<Button
					variant="secondary"
					onclick={() => displayInfo('Components are in /lib/components/ui/')}
				>
					View Component Files
				</Button>
			</div>
		</div>
	</Card>

	<!-- Benefits -->
	<Card title="✨ Benefits of These Components">
		<div class="grid gap-4 md:grid-cols-3">
			<div class="text-center p-4">
				<div class="text-2xl mb-2">🎨</div>
				<h4 class="font-semibold text-gray-900 dark:text-white">Professional Design</h4>
				<p class="text-xs text-gray-600 dark:text-gray-400 mt-1">
					Enterprise-grade UI patterns from Tailwind UI
				</p>
			</div>
			<div class="text-center p-4">
				<div class="text-2xl mb-2">♿</div>
				<h4 class="font-semibold text-gray-900 dark:text-white">Accessibility</h4>
				<p class="text-xs text-gray-600 dark:text-gray-400 mt-1">
					ARIA labels, keyboard navigation, focus management
				</p>
			</div>
			<div class="text-center p-4">
				<div class="text-2xl mb-2">📱</div>
				<h4 class="font-semibold text-gray-900 dark:text-white">Responsive</h4>
				<p class="text-xs text-gray-600 dark:text-gray-400 mt-1">
					Mobile-first design with proper breakpoints
				</p>
			</div>
		</div>
	</Card>
</div>
