<script>
    const { tablename, filter } = $props()

    console.log(tablename, filter)

</script>

<h1>Bar Chart</h1>
<p>{tablename}</p>
<p>{JSON.stringify(filter)}</p>