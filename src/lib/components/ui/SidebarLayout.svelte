<!--
  Modern Sidebar Layout with Tailwind UI patterns
  Features:
  - Responsive sidebar (collapsible on mobile)
  - Modern design with hover states
  - Active state indicators
  - User profile section
  - Mobile hamburger menu
-->

<script lang="ts">
	import { page } from '$app/stores'
	import { fly, fade } from 'svelte/transition'
	import ThemeToggle from './ThemeToggle.svelte'

	interface NavItem {
		name: string
		href: string
		icon: string
		badge?: number
	}

	interface UserProfile {
		name?: string
		email?: string
		avatar?: string
	}

	interface Props {
		navItems?: NavItem[]
		userProfile?: UserProfile
		appName?: string
		children?: import('svelte').Snippet
	}

	let {
		navItems = [],
		userProfile = {},
		appName = 'Mental Health Tracker',
		children
	}: Props = $props()

	let sidebarOpen = $state(false)

	// Check if a nav item is active
	function isActive(itemPath: string): boolean {
		const currentPath = $page.url.pathname
		if (itemPath === '/') {
			return currentPath === '/'
		}
		return currentPath.startsWith(itemPath)
	}
</script>

<div class="h-screen flex">
	<!-- Mobile sidebar backdrop -->
	{#if sidebarOpen}
		<div
			class="fixed inset-0 z-50 bg-gray-900/80 lg:hidden"
			transition:fade={{ duration: 150 }}
			onclick={() => (sidebarOpen = false)}
			role="button"
			tabindex="0"
			aria-label="Close sidebar"
			onkeydown={(e) => e.key === 'Escape' && (sidebarOpen = false)}
		></div>
	{/if}

	<!-- Sidebar -->
	<div
		class="fixed inset-y-0 left-0 z-50 flex w-72 flex-col lg:fixed lg:inset-y-0 lg:z-50 lg:flex {sidebarOpen
			? 'translate-x-0'
			: '-translate-x-full lg:translate-x-0'}"
		transition:fly={{ x: -288, duration: 300 }}
	>
		<!-- Sidebar component -->
		<div class="flex grow flex-col gap-y-5 overflow-y-auto bg-white dark:bg-gray-900 border-r border-gray-200 dark:border-gray-700">
			<div class="flex h-16 shrink-0 items-center px-6 border-b border-gray-200 dark:border-gray-700">
				<h2 class="text-xl font-semibold text-gray-900 dark:text-white">{appName}</h2>
			</div>

			<nav class="flex flex-1 flex-col px-6">
				<ul role="list" class="flex flex-1 flex-col gap-y-7">
					<li>
						<ul role="list" class="-mx-2 space-y-1">
							{#each navItems as item (item.href)}
								<li>
									<a
										href={item.href}
										class="group flex gap-x-3 rounded-md p-2 text-sm leading-6 font-semibold transition-colors
											{isActive(item.href)
											? 'bg-blue-50 dark:bg-blue-900/20 text-blue-600 dark:text-blue-400'
											: 'text-gray-700 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white hover:bg-gray-50 dark:hover:bg-gray-800'}"
										data-sveltekit-preload-data="hover"
									>
										<span
											class="flex h-6 w-6 shrink-0 items-center justify-center text-gray-400 group-hover:text-gray-600 dark:group-hover:text-gray-300
												{isActive(item.href) ? 'text-blue-600 dark:text-blue-400' : ''}"
										>
											{#if item.icon === 'home'}
												<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"/>
												</svg>
											{:else if item.icon === 'clipboard'}
												<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"/>
												</svg>
											{:else if item.icon === 'emoticon'}
												<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14.828 14.828a4 4 0 01-5.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
												</svg>
											{:else if item.icon === 'chart'}
												<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"/>
												</svg>
											{:else}
												<svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"/>
													<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/>
												</svg>
											{/if}
										</span>
										{item.name}
										{#if item.badge}
											<span
												class="ml-auto w-9 min-w-max whitespace-nowrap rounded-full bg-gray-900 dark:bg-gray-700 px-2.5 py-0.5 text-center text-xs font-medium leading-5 text-white"
												aria-label="{item.badge} new items"
											>
												{item.badge}
											</span>
										{/if}
									</a>
								</li>
							{/each}
						</ul>
					</li>

					<li class="mt-auto">
						<div class="border-t border-gray-200 dark:border-gray-700 pt-4">
							<!-- User profile section -->
							{#if userProfile.name || userProfile.email}
								<div class="flex items-center gap-x-4 px-2 py-3 text-sm font-semibold leading-6 text-gray-900 dark:text-white">
									<div
										class="h-8 w-8 rounded-full bg-gray-300 dark:bg-gray-700 flex items-center justify-center"
									>
										{#if userProfile.avatar}
											<img
												class="h-8 w-8 rounded-full"
												src={userProfile.avatar}
												alt={userProfile.name || 'User'}
											/>
										{:else}
											<span class="text-sm font-medium text-gray-600 dark:text-gray-400">
												{userProfile.name?.charAt(0) || 'U'}
											</span>
										{/if}
									</div>
									<div class="flex-1">
										{#if userProfile.name}
											<p class="text-sm font-semibold text-gray-900 dark:text-white">
												{userProfile.name}
											</p>
										{/if}
										{#if userProfile.email}
											<p class="text-xs text-gray-500 dark:text-gray-400">{userProfile.email}</p>
										{/if}
									</div>
								</div>
							{/if}

							<!-- Theme toggle -->
							<div class="flex items-center justify-between px-2 py-3">
								<span class="text-sm font-medium text-gray-700 dark:text-gray-300">Theme</span>
								<ThemeToggle />
							</div>
						</div>
					</li>
				</ul>
			</nav>
		</div>
	</div>

	<!-- Main content area -->
	<div class="lg:pl-72 flex-1">
		<!-- Top bar for mobile -->
		<div
			class="sticky top-0 z-40 flex h-16 shrink-0 items-center gap-x-4 border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900 px-4 shadow-sm sm:gap-x-6 sm:px-6 lg:hidden"
		>
			<button
				type="button"
				class="-m-2.5 p-2.5 text-gray-700 dark:text-gray-300 lg:hidden"
				onclick={() => (sidebarOpen = !sidebarOpen)}
				aria-label="Open sidebar"
			>
				<svg class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor">
					<path
						stroke-linecap="round"
						stroke-linejoin="round"
						d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25h16.5"
					/>
				</svg>
			</button>

			<div class="flex flex-1 gap-x-4 self-stretch lg:gap-x-6">
				<div class="flex items-center gap-x-4 lg:gap-x-6">
					<h1 class="text-lg font-semibold text-gray-900 dark:text-white">{appName}</h1>
				</div>
			</div>
		</div>

		<!-- Page content -->
		<main class="py-10">
			<div class="px-4 sm:px-6 lg:px-8">
				{@render children?.()}
			</div>
		</main>
	</div>
</div>