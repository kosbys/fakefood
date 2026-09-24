import { env } from '$env/dynamic/private';
import type { ProductType } from '$lib/types/product';
import type { PageServerLoad } from './$types';


// also get categories
export const load: PageServerLoad = async ({ fetch }) => {
    const res = await fetch(`${env.API_URL}/products`);

    if (!res.ok) {
        throw new Error('Server error fetching products');
    }

    const products = await res.json();

    const categories = [
      ...new Set(
        products.map((product: ProductType) => product.category)
      )
    ];

    return {
      products,
      categories
    };
};
