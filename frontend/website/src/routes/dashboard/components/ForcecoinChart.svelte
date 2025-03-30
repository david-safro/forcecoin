<script lang="ts">
    import { onMount } from 'svelte';


    export let series: { name: string; data: number[] }[] = [];
    export let categories: string[] = [];


    let ApexCharts: any = null;

    const options = {
        chart: { type: 'line' },
        xaxis: { categories },
    };


    onMount(async () => {
        const module = await import('svelte-apexcharts');
        ApexCharts = module.default;
    });
</script>

{#if ApexCharts}
    <svelte:component this={ApexCharts} {options} {series} type="line" height={250} />
{/if}
