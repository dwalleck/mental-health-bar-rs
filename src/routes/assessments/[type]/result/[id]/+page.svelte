<script lang="ts">
	import { page } from '$app/stores';
	import AssessmentResults from '$lib/components/assessments/AssessmentResults.svelte';

	$: assessmentId = parseInt($page.params.id);
</script>

<AssessmentResults {assessmentId} />
