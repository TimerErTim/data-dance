const env = process.env.NODE_ENV

const config = {
    host: env === 'development' ? 'http://localhost:3000' : '',
}

export default config
