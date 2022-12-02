export default {
    port: 3000,
    fetch: async (request) => {
        const chunks = [];
        for await (let chunk of request.body) {
            chunks.push(chunk)
        }
        const buffer = Buffer.concat(chunks);
        console.log(buffer.toString("utf-8"));
    }
};
