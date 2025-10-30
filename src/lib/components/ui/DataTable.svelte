<!--
  Professional Data Table with Tailwind UI patterns
  Features:
  - Sortable columns
  - Pagination
  - Row selection
  - Responsive design
  - Empty states
  - Loading states
  - Actions column
-->

<script lang="ts" generics="T">
	import LoadingSpinner from './LoadingSpinner.svelte'

	interface Column<T> {
		key: string
		label: string
		sortable?: boolean
		align?: 'left' | 'center' | 'right'
		render?: (item: T) => string
		width?: string
	}

	interface Action<T> {
		label: string
		icon?: string
		onClick: (item: T) => void
		variant?: 'primary' | 'secondary' | 'danger'
		show?: (item: T) => boolean
	}

	interface Props<T> {
		columns?: Column<T>[]
		data?: T[]
		actions?: Action<T>[]
		loading?: boolean
		emptyMessage?: string
		selectable?: boolean
		selectedItems?: T[]
		keyField?: string
		striped?: boolean
		pagination?: boolean
		pageSize?: number
		sortKey?: string
		sortOrder?: 'asc' | 'desc'
	}

	let {
		columns = [],
		data = [],
		actions = [],
		loading = false,
		emptyMessage = 'No data found',
		selectable = false,
		selectedItems = $bindable([]),
		keyField = 'id',
		striped = true,
		pagination = true,
		pageSize = 10,
		sortKey = $bindable(''),
		sortOrder = $bindable('asc' as 'asc' | 'desc')
	}: Props<T> = $props()

	let currentPage = $state(1)
	let selectAll = $state(false)

	// Computed values
	const totalPages = $derived(Math.ceil(data.length / pageSize))
	const paginatedData = $derived(
		pagination
			? data.slice((currentPage - 1) * pageSize, currentPage * pageSize)
			: data
	)

	// Sorting
	function handleSort(column: Column<T>) {
		if (!column.sortable) return

		if (sortKey === column.key) {
			sortOrder = sortOrder === 'asc' ? 'desc' : 'asc'
		} else {
			sortKey = column.key
			sortOrder = 'asc'
		}
	}

	// Selection
	function toggleSelectAll() {
		if (selectAll) {
			selectedItems = [...paginatedData]
		} else {
			selectedItems = []
		}
	}

	function toggleSelect(item: T) {
		const index = selectedItems.findIndex((i) => i[keyField] === item[keyField])
		if (index >= 0) {
			selectedItems = selectedItems.filter((_, i) => i !== index)
		} else {
			selectedItems = [...selectedItems, item]
		}
	}

	function isSelected(item: T): boolean {
		return selectedItems.some((i) => i[keyField] === item[keyField])
	}

	// Pagination
	function goToPage(page: number) {
		currentPage = Math.max(1, Math.min(page, totalPages))
	}

	$effect(() => {
		selectAll = paginatedData.length > 0 && paginatedData.every((item) => isSelected(item))
	})
</script>

<div class="flow-root">
	<div class="-mx-4 -my-2 overflow-x-auto sm:-mx-6 lg:-mx-8">
		<div class="inline-block min-w-full py-2 align-middle sm:px-6 lg:px-8">
			<div class="overflow-hidden shadow ring-1 ring-black ring-opacity-5 sm:rounded-lg">
				<table class="min-w-full divide-y divide-gray-300 dark:divide-gray-700">
					<thead class="bg-gray-50 dark:bg-gray-800">
						<tr>
							{#if selectable}
								<th scope="col" class="relative px-7 sm:w-12 sm:px-6">
									<input
										type="checkbox"
										class="absolute left-4 top-1/2 -mt-2 h-4 w-4 rounded border-gray-300 dark:border-gray-700 text-blue-600 focus:ring-blue-600"
										bind:checked={selectAll}
										onchange={toggleSelectAll}
										aria-label="Select all"
									/>
								</th>
							{/if}

							{#each columns as column (column.key)}
								<th
									scope="col"
									class="px-3 py-3.5 text-left text-sm font-semibold text-gray-900 dark:text-gray-100
										{column.align === 'center' ? 'text-center' : ''}
										{column.align === 'right' ? 'text-right' : ''}
										{column.sortable ? 'cursor-pointer select-none hover:bg-gray-100 dark:hover:bg-gray-700' : ''}"
									style="width: {column.width || 'auto'}"
									onclick={() => handleSort(column)}
								>
									<div class="flex items-center gap-2">
										<span>{column.label}</span>
										{#if column.sortable}
											<span class="text-gray-400">
												{#if sortKey === column.key}
													{#if sortOrder === 'asc'}
														<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
															<path
																fill-rule="evenodd"
																d="M5.293 7.293a1 1 0 011.414 0L10 10.586l3.293-3.293a1 1 0 111.414 1.414l-4 4a1 1 0 01-1.414 0l-4-4a1 1 0 010-1.414z"
																clip-rule="evenodd"
															/>
														</svg>
													{:else}
														<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
															<path
																fill-rule="evenodd"
																d="M14.707 12.707a1 1 0 01-1.414 0L10 9.414l-3.293 3.293a1 1 0 01-1.414-1.414l4-4a1 1 0 011.414 0l4 4a1 1 0 010 1.414z"
																clip-rule="evenodd"
															/>
														</svg>
													{/if}
												{:else}
													<svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
														<path
															fill-rule="evenodd"
															d="M10 3a1 1 0 01.707.293l3 3a1 1 0 01-1.414 1.414L10 5.414 7.707 7.707a1 1 0 01-1.414-1.414l3-3A1 1 0 0110 3zm-3.707 9.293a1 1 0 011.414 0L10 14.586l2.293-2.293a1 1 0 011.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z"
															clip-rule="evenodd"
														/>
													</svg>
												{/if}
											</span>
										{/if}
									</div>
								</th>
							{/each}

							{#if actions.length > 0}
								<th scope="col" class="relative py-3.5 pl-3 pr-4 sm:pr-6">
									<span class="sr-only">Actions</span>
								</th>
							{/if}
						</tr>
					</thead>
					<tbody class="divide-y divide-gray-200 dark:divide-gray-700 bg-white dark:bg-gray-900">
						{#if loading}
							<tr>
								<td
									colspan={columns.length + (selectable ? 1 : 0) + (actions.length > 0 ? 1 : 0)}
									class="px-3 py-8 text-center text-sm text-gray-500 dark:text-gray-400"
								>
									<LoadingSpinner />
									<p class="mt-2">Loading data...</p>
								</td>
							</tr>
						{:else if paginatedData.length === 0}
							<tr>
								<td
									colspan={columns.length + (selectable ? 1 : 0) + (actions.length > 0 ? 1 : 0)}
									class="px-3 py-8 text-center text-sm text-gray-500 dark:text-gray-400"
								>
									{emptyMessage}
								</td>
							</tr>
						{:else}
							{#each paginatedData as item, index (item[keyField])}
								<tr class="{striped && index % 2 === 0 ? 'bg-gray-50 dark:bg-gray-800/50' : ''}">
									{#if selectable}
										<td class="relative px-7 sm:w-12 sm:px-6">
											<input
												type="checkbox"
												class="absolute left-4 top-1/2 -mt-2 h-4 w-4 rounded border-gray-300 dark:border-gray-700 text-blue-600 focus:ring-blue-600"
												checked={isSelected(item)}
												onchange={() => toggleSelect(item)}
												aria-label="Select row"
											/>
										</td>
									{/if}

									{#each columns as column (column.key)}
										<td
											class="whitespace-nowrap px-3 py-4 text-sm text-gray-900 dark:text-gray-100
												{column.align === 'center' ? 'text-center' : ''}
												{column.align === 'right' ? 'text-right' : ''}"
										>
											{#if column.render}
												{@html column.render(item)}
											{:else}
												{item[column.key]}
											{/if}
										</td>
									{/each}

									{#if actions.length > 0}
										<td class="relative whitespace-nowrap py-4 pl-3 pr-4 text-right text-sm font-medium sm:pr-6">
											<div class="flex justify-end gap-2">
												{#each actions as action (action.label)}
													{#if !action.show || action.show(item)}
														<button
															type="button"
															class="text-blue-600 hover:text-blue-900 dark:text-blue-400 dark:hover:text-blue-300
																{action.variant === 'danger' ? 'text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300' : ''}
																{action.variant === 'secondary' ? 'text-gray-600 hover:text-gray-900 dark:text-gray-400 dark:hover:text-gray-300' : ''}"
															onclick={() => action.onClick(item)}
														>
															{action.label}
														</button>
													{/if}
												{/each}
											</div>
										</td>
									{/if}
								</tr>
							{/each}
						{/if}
					</tbody>
				</table>

				{#if pagination && totalPages > 1}
					<nav
						class="flex items-center justify-between border-t border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 px-4 py-3 sm:px-6"
						aria-label="Pagination"
					>
						<div class="hidden sm:block">
							<p class="text-sm text-gray-700 dark:text-gray-300">
								Showing
								<span class="font-medium">{(currentPage - 1) * pageSize + 1}</span>
								to
								<span class="font-medium">
									{Math.min(currentPage * pageSize, data.length)}
								</span>
								of
								<span class="font-medium">{data.length}</span>
								results
							</p>
						</div>
						<div class="flex flex-1 justify-between sm:justify-end">
							<button
								class="relative inline-flex items-center rounded-md bg-white dark:bg-gray-800 px-3 py-2 text-sm font-semibold text-gray-900 dark:text-gray-100 ring-1 ring-inset ring-gray-300 dark:ring-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
								onclick={() => goToPage(currentPage - 1)}
								disabled={currentPage === 1}
							>
								Previous
							</button>
							<button
								class="relative ml-3 inline-flex items-center rounded-md bg-white dark:bg-gray-800 px-3 py-2 text-sm font-semibold text-gray-900 dark:text-gray-100 ring-1 ring-inset ring-gray-300 dark:ring-gray-700 hover:bg-gray-50 dark:hover:bg-gray-700 disabled:opacity-50 disabled:cursor-not-allowed"
								onclick={() => goToPage(currentPage + 1)}
								disabled={currentPage === totalPages}
							>
								Next
							</button>
						</div>
					</nav>
				{/if}
			</div>
		</div>
	</div>
</div>