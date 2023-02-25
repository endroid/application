<svelte:head>
	<link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.0-alpha1/dist/css/bootstrap.min.css" rel="stylesheet">
	<title>Endroid</title>
	<meta name="description" content="SvelteKit" />
</svelte:head>

<script>
	let text = 'QR Code contents';
</script>

<table>
	<tr>
		<td>
			<img src="https://localhost:9000/qr-code/default/{text}" alt="QR code" />
		</td>
		<td>
			Generate a QR code
			<p><input bind:value={text} /></p>
		</td>
	</tr>
</table>
