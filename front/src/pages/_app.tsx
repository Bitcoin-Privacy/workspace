import React, { useMemo } from "react";
import Head from "next/head";
import type { AppProps } from "next/app";
import theme from "../lib/theme";
import type { EmotionCache } from "@emotion/cache";
import createEmotionCache from "../lib/create-emotion-cache";
import { CacheProvider } from "@emotion/react";
import { Box, Center, ChakraProvider, Spinner } from "@chakra-ui/react";
import "../styles/variables.css";
import "../styles/global.css";
import { QueryClient, QueryClientProvider } from "react-query";
import { useInit } from "@/hooks";

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

  const { appState } = useInit();

  const LoadingRender = useMemo(() => {
    return (
      <Center
        zIndex="1000"
        position="fixed"
        left="0"
        top="0"
        w="100%"
        h="100%"
        bg="bg.secondary"
      >
        <Box position="relative">
          <Center
            position="absolute"
            top="50%"
            left="50%"
            transform="translate(-50%,-50%)"
          >
            <Spinner w="60px" h="60px" color="text.secondary" />
          </Center>
        </Box>
      </Center>
    );
  }, []);

  const MainComponent = useMemo(() => {
    return <Component {...pageProps} />;
  }, [pageProps, Component]);

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
          {appState.loading.value && LoadingRender}
          {appState.ready.value && MainComponent}
          {appState.ready.value && <>{/* Modals */}</>}
        </ChakraProvider>
      </CacheProvider>
    </QueryClientProvider>
  );
}
