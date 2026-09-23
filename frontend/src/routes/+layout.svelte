<script lang="ts">
    import favicon from "$lib/assets/favicon.svg";
    import Footer from "$lib/components/Footer.svelte";
    import Header from "$lib/components/Header.svelte";
    import "../app.css"

    let { children } = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon} />
</svelte:head>


<div class="max-w-2/4 items-center m-auto">
    <Header/>
    {@render children()}
</div>
<Footer/>
