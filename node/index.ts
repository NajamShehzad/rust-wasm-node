import { add, get_image_base64_from_url } from './pkg/rust_lib';

async function run() {
    // Initialize the wasm module

    // Call the add function
    const result = add(5, 3);
    console.log(`Result of add(5, 3): ${result}`);

    // Example URL of an image (you should replace it with an actual image URL)
    const imageUrl = 'https://upload.wikimedia.org/wikipedia/commons/f/f3/Stora_torget%2C_Link%C3%B6ping%2C_juli_2005.jpg';

    // Call the get_image_base64_from_url function
    try {
        const imageBase64 = await get_image_base64_from_url(imageUrl);
        console.log(`Base64 image data: ${imageBase64}`);
    } catch (error) {
        console.error(`Error fetching image: ${error}`);
    }
}

run();
