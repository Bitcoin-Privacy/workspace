import React from "react";
import Head from "next/head";
import type { AppProps } from "next/app";
import theme from "../lib/theme";
import type { EmotionCache } from "@emotion/cache";
import createEmotionCache from "../lib/create-emotion-cache";
import { CacheProvider } from "@emotion/react";
import { ChakraProvider } from "@chakra-ui/react";
import "../styles/variables.css";
import "../styles/global.css";
import { QueryClient, QueryClientProvider } from "react-query";

const clientSideEmotionCache = createEmotionCache();
const queryClient = new QueryClient();

type MyAppProps = AppProps & {
  emotionCache?: EmotionCache;
};

export default function MyApp(props: MyAppProps) {
  const {
    Component,
    pageProps: { session, ...pageProps },
    emotionCache = clientSideEmotionCache,
  } = props;

  return (
    <QueryClientProvider client={queryClient}>
      <CacheProvider value={emotionCache}>
        <Head>
          <meta
            name="viewport"
            content="minimum-scale=1, initial-scale=1, width=device-width"
          />
        </Head>

        <ChakraProvider theme={theme}>
          <Component {...pageProps} />
        </ChakraProvider>
      </CacheProvider>
    </QueryClientProvider>
  );
}
