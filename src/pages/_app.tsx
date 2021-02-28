import React from 'react'
import { AppProps } from 'next/dist/pages/_app'
import '@src/styles/globals.css'

const App: React.FC<AppProps> = ({ Component, pageProps }) => {
  return <Component {...pageProps} />
}

export default App
