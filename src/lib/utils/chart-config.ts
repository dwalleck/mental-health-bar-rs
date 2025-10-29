// T135: Chart.js configuration defaults
import {
	Chart as ChartJS,
	CategoryScale,
	LinearScale,
	PointElement,
	LineElement,
	BarElement,
	Title,
	Tooltip,
	Legend,
	Filler,
	type ChartOptions,
} from 'chart.js'
import annotationPlugin from 'chartjs-plugin-annotation'

// Register Chart.js components
ChartJS.register(
	CategoryScale,
	LinearScale,
	PointElement,
	LineElement,
	BarElement,
	Title,
	Tooltip,
	Legend,
	Filler,
	annotationPlugin
)

// Default chart options (shared base configuration)
const baseChartOptions = {
	responsive: true,
	maintainAspectRatio: true,
	aspectRatio: 2,
	animation: {
		duration: 500,
	},
	interaction: {
		mode: 'index' as const,
		intersect: false,
	},
	plugins: {
		legend: {
			display: true,
			position: 'top' as const,
		},
		tooltip: {
			enabled: true,
			backgroundColor: 'rgba(0, 0, 0, 0.8)',
			padding: 12,
			titleFont: {
				size: 14,
				weight: 'bold' as const,
			},
			bodyFont: {
				size: 13,
			},
		},
	},
	scales: {
		y: {
			beginAtZero: true,
			grid: {
				color: 'rgba(0, 0, 0, 0.1)',
			},
		},
		x: {
			grid: {
				display: false,
			},
		},
	},
}

// Default chart options for line charts
export const defaultChartOptions: ChartOptions<'line'> = baseChartOptions

// Default bar chart options
export const defaultBarChartOptions: ChartOptions<'bar'> = {
	...baseChartOptions,
	indexAxis: 'y', // Horizontal bars
	plugins: {
		...baseChartOptions.plugins,
		legend: {
			display: false,
		},
	},
}

// Color palette for mood ratings
export const moodColors = {
	1: '#EF4444', // Very Bad - red
	2: '#F97316', // Bad - orange
	3: '#F59E0B', // Neutral - amber
	4: '#84CC16', // Good - lime
	5: '#22C55E', // Very Good - green
}

// Severity colors for assessment thresholds
export const severityColors = {
	minimal: '#22C55E', // green
	mild: '#F59E0B', // amber
	moderate: '#F97316', // orange
	moderately_severe: '#EF4444', // red
	severe: '#DC2626', // dark red
}

// Create threshold annotation for Chart.js
export function createThresholdAnnotation(label: string, value: number, color: string) {
	return {
		type: 'line' as const,
		yMin: value,
		yMax: value,
		borderColor: color,
		borderWidth: 2,
		borderDash: [5, 5],
		label: {
			display: true,
			content: label,
			position: 'end' as const,
			backgroundColor: color,
			color: '#fff',
			padding: 4,
			font: {
				size: 11,
				weight: 'bold' as const,
			},
		},
	}
}
