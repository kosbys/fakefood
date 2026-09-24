

<script lang="ts">
    import type { ProductType } from "$lib/types/product";

    // filter using search/categories
    import Product from "./Product.svelte";

    type ProductListProps = {
      search: string,
      currentCategory: string,
      products: ProductType[]
    };


    let { search, currentCategory, products }: ProductListProps = $props();

    // get products and filter them
    const filteredProducts = $derived(
      products.filter((product) => {
        const matchedSearch = product.name.toLowerCase().includes(search.toLowerCase()) && search !== "";

        const matchedCategory = currentCategory === "All" || product.category === currentCategory;

        return matchedCategory && matchedSearch;
      })
    )
    // foreach products, props on products
</script>

<div class="flex flex-row flex-wrap justify-between items-center gap-8 py-4">
    {#each filteredProducts as product }
        <div>a</div>
    {/each}
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />
    <Product />

</div>
