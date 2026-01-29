<script lang="ts">
	import { processingState } from '../stores/processing';
	import type { DataRow } from '../types';

	function exportToJSON(data: DataRow[]) {
		const json = JSON.stringify(data, null, 2);
		downloadFile(json, 'processed-data.json', 'application/json');
	}

	function exportToCSV(data: DataRow[]) {
		if (data.length === 0) return;

		const headers = Object.keys(data[0]);
		const csvRows = [headers.join(',')];

		for (const row of data) {
			const values = headers.map((header) => {
				const value = row[header];
				const stringValue = value?.toString() || '';
				return stringValue.includes(',') ? `"${stringValue}"` : stringValue;
			});
			csvRows.push(values.join(','));
		}

		const csv = csvRows.join('\n');
		downloadFile(csv, 'processed-data.csv', 'text/csv');
	}

	function downloadFile(content: string, filename: string, mimeType: string) {
		const blob = new Blob([content], { type: mimeType });
		const url = URL.createObjectURL(blob);
		const link = document.createElement('a');
		link.href = url;
		link.download = filename;
		document.body.appendChild(link);
		link.click();
		document.body.removeChild(link);
		URL.revokeObjectURL(url);
	}

	$: hasResult = $processingState.result !== null;
	$: resultData = $processingState.result || [];
	$: headers = resultData.length > 0 ? Object.keys(resultData[0]) : [];
</script>

{#if hasResult}
	<div class="bg-white rounded-lg shadow-sm border border-gray-200 p-6 animate-slide-up">
		<div class="flex justify-between items-center mb-4">
			<div>
				<h2 class="text-lg font-semibold text-gray-900">Processing Results</h2>
				<p class="text-sm text-gray-500 mt-1">{resultData.length} rows</p>
			</div>
			<div class="flex space-x-2">
				<button
					on:click={() => exportToJSON(resultData)}
					class="px-4 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors duration-200 flex items-center space-x-2"
				>
					<svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
					</svg>
					<span>Export JSON</span>
				</button>
				<button
					on:click={() => exportToCSV(resultData)}
					class="px-4 py-2 text-sm font-medium text-white bg-primary-600 rounded-lg hover:bg-primary-700 transition-colors duration-200 flex items-center space-x-2"
				>
					<svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
					</svg>
					<span>Export CSV</span>
				</button>
			</div>
		</div>

		<div class="overflow-x-auto border border-gray-200 rounded-lg">
			<div class="max-h-96 overflow-y-auto">
				<table class="min-w-full divide-y divide-gray-200">
					<thead class="bg-gray-50 sticky top-0">
						<tr>
							{#each headers as header}
								<th
									scope="col"
									class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider whitespace-nowrap"
								>
									{header}
								</th>
							{/each}
						</tr>
					</thead>
					<tbody class="bg-white divide-y divide-gray-200">
						{#each resultData as row, index}
							<tr class="hover:bg-gray-50 transition-colors duration-150">
								{#each headers as header}
									<td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
										{#if typeof row[header] === 'number'}
											<span class="font-mono">{row[header]}</span>
										{:else}
											{row[header]}
										{/if}
									</td>
								{/each}
							</tr>
						{/each}
					</tbody>
				</table>
			</div>
		</div>

		{#if resultData.length === 0}
			<div class="text-center py-8">
				<svg class="mx-auto h-12 w-12 text-gray-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
				</svg>
				<p class="mt-2 text-sm text-gray-500">No data to display</p>
			</div>
		{/if}

		<div class="mt-4 p-4 bg-blue-50 border border-blue-200 rounded-lg">
			<div class="flex items-start space-x-3">
				<svg class="w-5 h-5 text-blue-500 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
					<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
				</svg>
				<div class="flex-1">
					<h3 class="text-sm font-medium text-blue-800">Deterministic Guarantee</h3>
					<p class="text-sm text-blue-700 mt-1">
						These results are deterministic and reproducible. Running the same transformation with the same input data will always produce identical outputs.
					</p>
				</div>
			</div>
		</div>
	</div>
{/if}
