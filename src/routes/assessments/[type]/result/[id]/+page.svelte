<script lang="ts">
	import { page } from '$app/stores'
	import AssessmentResults from '$lib/components/assessments/AssessmentResults.svelte'

	$: assessmentId = parseInt($page.params.id || '0')
</script>

<AssessmentResults {assessmentId} />
