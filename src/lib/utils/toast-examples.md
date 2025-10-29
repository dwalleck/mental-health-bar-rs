# Toast Notification System - Usage Examples

## Overview

The toast notification system provides non-blocking, user-friendly feedback for actions throughout the app.

## Basic Usage

```typescript
import { toastStore } from '$lib/stores/toast'

// Show a success message
toastStore.success('Assessment saved successfully!')

// Show an error message
toastStore.error('Failed to load data')

// Show a warning message
toastStore.warning('This action cannot be undone')

// Show an info message
toastStore.info('New feature available')
```

## Advanced Usage

### Custom Duration

```typescript
// Auto-dismiss after 3 seconds (default is 5 seconds)
toastStore.success('Quick message', 3000)

// Never auto-dismiss (duration = 0)
toastStore.error('Critical error - please contact support', 0)

// Longer duration for important messages
toastStore.warning('Session will expire in 5 minutes', 10000)
```

### Generic Show Method

```typescript
// Full control over all parameters
toastStore.show('Custom message', 'info', 8000)
```

### Programmatic Dismissal

```typescript
// Get the toast ID when showing
const id = toastStore.error('Failed to connect')

// Dismiss it later programmatically
setTimeout(() => {
	toastStore.dismiss(id)
}, 2000)
```

### Clear All Toasts

```typescript
// Clear all visible toasts at once
toastStore.clear()
```

## Real-World Examples

### Form Submission Success

```typescript
async function submitForm() {
	try {
		await invoke('save_data', { data })
		toastStore.success('Form submitted successfully!')
	} catch (error) {
		toastStore.error(`Failed to submit: ${error}`)
	}
}
```

### Retry with Feedback

```typescript
async function loadData() {
	try {
		const data = await invoke('fetch_data')
		return data
	} catch (error) {
		toastStore.error('Failed to load data - retrying...')
		// Retry logic here
	}
}
```

### Multi-step Process

```typescript
async function processData() {
	toastStore.info('Starting data processing...')

	try {
		await step1()
		toastStore.success('Step 1 complete')

		await step2()
		toastStore.success('Step 2 complete')

		await step3()
		toastStore.success('All steps completed!')
	} catch (error) {
		toastStore.error(`Process failed: ${error}`)
	}
}
```

### Warning Before Action

```typescript
function deleteItem() {
	toastStore.warning('Are you sure? This action cannot be undone', 0)
	// Show confirmation dialog...
}
```

## Integration with Existing Error Handling

You can use toasts alongside existing inline error messages:

```typescript
// Show inline error for immediate feedback
error = 'Invalid input'

// Also show toast for non-blocking notification
toastStore.error('Please correct the errors in the form')
```

Or replace inline messages entirely with toasts for a cleaner UI:

```typescript
// Before (inline):
let errorMessage = $state<string | null>(null)
errorMessage = 'Failed to save'

// After (toast):
toastStore.error('Failed to save')
```

## Best Practices

1. **Success messages** - Keep them brief and positive
   - ✅ "Saved successfully!"
   - ❌ "Your data has been successfully saved to the database"

2. **Error messages** - Be specific and actionable
   - ✅ "Failed to connect. Please check your internet connection."
   - ❌ "Error occurred"

3. **Duration** - Match to message importance
   - Quick confirmations: 3000ms
   - Standard messages: 5000ms (default)
   - Important warnings: 8000-10000ms
   - Critical errors: 0 (manual dismiss)

4. **Type selection**
   - `success` - Completed actions (save, delete, create)
   - `error` - Failed operations, validation errors
   - `warning` - Cautionary messages, potential issues
   - `info` - General information, tips, updates

5. **Don't overuse** - Too many toasts can be overwhelming
   - Avoid showing toasts for every single action
   - Group related notifications when possible
   - Use inline feedback for form validation

## Accessibility

The toast system is fully accessible:
- ARIA live region announces new toasts to screen readers
- Each toast is marked as an alert
- Dismiss buttons have proper labels
- Keyboard navigable (Tab to button, Enter to dismiss)
