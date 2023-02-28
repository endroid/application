<script>
	import { onMount } from 'svelte';
	let qrCodeData;

	onMount(async () => {
		const response = await fetch(`https://localhost/api/users`);
		const data = await response.json();
		qrCodeData = JSON.stringify(data);
	});
</script>

<table>
    <tr>
        <td>
            <img src="https://localhost/qr-code/default/{qrCodeData}" alt="QR code" />
            <textarea bind:value={qrCodeData}></textarea>
        </td>
    </tr>
</table>
