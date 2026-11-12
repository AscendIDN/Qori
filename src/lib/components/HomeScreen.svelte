<script lang="ts">
	import StartPage from './StartPage.svelte';
	import type { SearchMode, AppStats } from '../types';

	let {
		onSearch = (query: string, mode: SearchMode) => {},
		stats
	}: {
		onSearch?: (query: string, mode: SearchMode) => void;
		stats?: AppStats;
	} = $props();
</script>

<StartPage {onSearch} {stats} />
