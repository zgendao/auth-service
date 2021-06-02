<script>
	import { ethers } from "ethers";

	window.ethereum.enable()
	const provider = new ethers.providers.Web3Provider(window.ethereum);
	console.log(provider);
	const signer = provider.getSigner();
	signer.signMessage("Hello World").then(signature => {
		console.log(signature);
	});
	console.log(signer.getAddress().then(address => {
		console.log(address);
	}));

	export let name;
</script>

<main>
	<h1>Hello {name}!</h1>
	<p>Visit the <a href="https://svelte.dev/tutorial">Svelte tutorial</a> to learn how to build Svelte apps.</p>
</main>

<style>
	main {
		text-align: center;
		padding: 1em;
		max-width: 240px;
		margin: 0 auto;
	}

	h1 {
		color: #ff3e00;
		text-transform: uppercase;
		font-size: 4em;
		font-weight: 100;
	}

	@media (min-width: 640px) {
		main {
			max-width: none;
		}
	}
</style>