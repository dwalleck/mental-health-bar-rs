<script lang="ts">
	import ScheduleForm from '$lib/components/scheduling/ScheduleForm.svelte'
	import ScheduleList from '$lib/components/scheduling/ScheduleList.svelte'

	let refreshSchedules = $state(0)

	function handleScheduleCreated() {
		// Increment to trigger refresh in ScheduleList
		refreshSchedules++
	}
</script>

<svelte:head>
	<title>Settings - Mental Health Tracker</title>
</svelte:head>

<div class="min-h-screen bg-gray-50 py-8">
	<div class="max-w-6xl mx-auto px-4">
		<div class="mb-8">
			<h1 class="text-3xl font-bold text-gray-900">Settings</h1>
			<p class="text-gray-600 mt-2">Configure assessment schedules and notification preferences</p>
		</div>

		<div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
			<!-- Schedule Form -->
			<div>
				<ScheduleForm onSuccess={handleScheduleCreated} />
			</div>

			<!-- Schedule List -->
			<div>
				<ScheduleList bind:refresh={refreshSchedules} />
			</div>
		</div>
	</div>
</div>
