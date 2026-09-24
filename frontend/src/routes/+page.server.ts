import { env } from '$env/dynamic/private';


export async function load({ fetch }) {

  const res = await fetch(`${env.API_URL}/products`);

  if (!res.ok) {
    throw new Error("Server error fetching products");
  }

  return {
    products: await res.json()
  };
};
