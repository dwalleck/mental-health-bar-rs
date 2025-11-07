<script lang="ts">
	import {
		Icon,
		Home,
		ChartBar,
		CheckCircle,
		XCircle,
		ExclamationTriangle,
	} from 'svelte-hero-icons'
</script>

<div class="container mx-auto p-8">
	<h1 class="text-3xl font-bold mb-6">Heroicons Test</h1>

	<div class="space-y-8">
		<!-- Solid Icons Test -->
		<section>
			<h2 class="text-2xl font-semibold mb-4">Solid Icons</h2>
			<div class="flex gap-4 items-center">
				<div class="flex items-center gap-2">
					<Icon src={Home} solid class="w-6 h-6 text-blue-600" />
					<span>Home</span>
				</div>
				<div class="flex items-center gap-2">
					<Icon src={ChartBar} solid class="w-6 h-6 text-green-600" />
					<span>ChartBar</span>
				</div>
				<div class="flex items-center gap-2">
					<Icon src={CheckCircle} solid class="w-6 h-6 text-green-600" />
					<span>CheckCircle</span>
				</div>
				<div class="flex items-center gap-2">
					<Icon src={XCircle} solid class="w-6 h-6 text-red-600" />
					<span>XCircle</span>
				</div>
				<div class="flex items-center gap-2">
					<Icon src={ExclamationTriangle} solid class="w-6 h-6 text-yellow-600" />
					<span>Warning</span>
				</div>
			</div>
		</section>

		<!-- Outline Icons Test -->
		<section>
			<h2 class="text-2xl font-semibold mb-4">Outline Icons (Default)</h2>
			<div class="flex gap-4 items-center">
				<div class="flex items-center gap-2">
					<Icon src={Home} class="w-6 h-6 text-blue-600" />
					<span>Home</span>
				</div>
				<div class="flex items-center gap-2">
					<Icon src={ChartBar} class="w-6 h-6 text-green-600" />
					<span>ChartBar</span>
				</div>
				<div class="flex items-center gap-2">
					<Icon src={CheckCircle} class="w-6 h-6 text-green-600" />
					<span>CheckCircle</span>
				</div>
			</div>
		</section>

		<!-- Icon Size Variants -->
		<section>
			<h2 class="text-2xl font-semibold mb-4">Size Variants</h2>
			<div class="flex gap-4 items-center">
				<Icon src={Home} solid class="w-4 h-4 text-blue-600" />
				<Icon src={Home} solid class="w-6 h-6 text-blue-600" />
				<Icon src={Home} solid class="w-8 h-8 text-blue-600" />
				<Icon src={Home} solid class="w-12 h-12 text-blue-600" />
				<Icon src={Home} solid class="w-16 h-16 text-blue-600" />
			</div>
		</section>

		<!-- Mini Icons Test -->
		<section>
			<h2 class="text-2xl font-semibold mb-4">Mini Icons</h2>
			<div class="flex gap-4 items-center">
				<Icon src={Home} mini class="w-5 h-5 text-blue-600" />
				<Icon src={ChartBar} mini class="w-5 h-5 text-green-600" />
				<Icon src={CheckCircle} mini class="w-5 h-5 text-green-600" />
			</div>
		</section>

		<!-- Dark Mode Test -->
		<section class="p-4 bg-gray-100 dark:bg-gray-800 rounded-lg">
			<h2 class="text-2xl font-semibold mb-4">Dark Mode Compatible</h2>
			<div class="flex gap-4">
				<Icon src={CheckCircle} solid class="w-8 h-8 text-green-600 dark:text-green-400" />
				<Icon src={XCircle} solid class="w-8 h-8 text-red-600 dark:text-red-400" />
			</div>
		</section>
	</div>
</div>
