<script lang="ts">
	import { page } from '$app/stores';
	import AssessmentForm from '$lib/components/assessments/AssessmentForm.svelte';

	$: assessmentCode = $page.params.type.toUpperCase();
</script>

<AssessmentForm {assessmentCode} />
