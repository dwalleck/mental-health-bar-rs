<script lang="ts">
	let {
		variant = 'primary',
		type = 'button',
		disabled = false,
		fullWidth = false,
		onclick,
		children,
	}: {
		variant?: 'primary' | 'secondary' | 'danger'
		type?: 'button' | 'submit' | 'reset'
		disabled?: boolean
		fullWidth?: boolean
		onclick?: (event: MouseEvent) => void
		children?: import('svelte').Snippet
	} = $props()

	const variantClasses = {
		primary: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-blue-500 disabled:bg-blue-300',
		secondary:
			'bg-gray-200 text-gray-800 hover:bg-gray-300 focus:ring-gray-400 disabled:bg-gray-100',
		danger: 'bg-red-600 text-white hover:bg-red-700 focus:ring-red-500 disabled:bg-red-300',
	}
</script>

<button
	{type}
	{disabled}
	{onclick}
	class="px-4 py-2 rounded-lg font-medium transition-colors focus:outline-hidden focus:ring-2 focus:ring-offset-2 disabled:cursor-not-allowed {variantClasses[
		variant
	]} {fullWidth ? 'w-full' : ''}"
>
	{@render children?.()}
</button>
