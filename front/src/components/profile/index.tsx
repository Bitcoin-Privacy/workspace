import React, { FC, useEffect, useState } from "react";
import {
  Tabs,
  TabList,
  TabPanels,
  Tab,
  TabPanel,
  TabIndicator,
} from "@chakra-ui/react";
import { useProfilePage } from "@/hooks";
import {
  CoinjoinList,
  StateChainList,
  UtxoList,
  StateChainTransferList,
} from "..";
import { useRouter } from "next/router";

interface IProfilePanel {}

enum ProfileTab {
  STATECHAIN = 0,
  STATECHAIN_TRANSFER = 1,
  UTXO = 2,
  COINJOIN = 3,
}

export const ProfilePannel: FC<IProfilePanel> = (props) => {
  const router = useRouter();
  const tab = (router.query.tab as string) ?? "UTXO";
  const {
    states: {
      deriv,
      listUtxoQuery,
      listCoinjoinRoomsQuery,
      listTransferStatecoinsQuery,
      listStatecoinsQuery,
    },
  } = useProfilePage();

  const [tabIndex, setTabIndex] = useState(0);

  const handleTabsChange = (index: number) => {
    setTabIndex(index);
  };

  useEffect(() => {
    setTabIndex(ProfileTab[tab as keyof typeof ProfileTab]);
  }, [tab]);

  return (
    <Tabs
      isFitted
      variant="unstyled"
      w="100%"
      index={tabIndex}
      onChange={handleTabsChange}
    >
      <TabList>
        <Tab fontSize="18px" fontWeight="200" color="#aaa">
          Statechain
        </Tab>
        <Tab fontSize="18px" fontWeight="200" color="#aaa">
          Statechain transfer
        </Tab>
        <Tab fontSize="18px" fontWeight="200" color="#aaa">
          UTXO
        </Tab>
        <Tab fontSize="18px" fontWeight="200" color="#aaa">
          CoinJoin
        </Tab>
      </TabList>
      <TabIndicator mt="-1.5px" height="2px" bg="cyan.200" borderRadius="1px" />
      <TabPanels>
        <TabPanel>
          <StateChainList
            isLoading={listStatecoinsQuery.isLoading}
            isError={listStatecoinsQuery.isError}
            deriv={deriv}
            data={listStatecoinsQuery.data ?? []}
          />
        </TabPanel>
        <TabPanel>
          <StateChainTransferList
            isLoading={listTransferStatecoinsQuery.isLoading}
            isError={listTransferStatecoinsQuery.isError}
            deriv={deriv}
            data={listTransferStatecoinsQuery.data ?? []}
          />
        </TabPanel>
        <TabPanel>
          <UtxoList
            isLoading={listUtxoQuery.isLoading}
            isError={listUtxoQuery.isError}
            data={listUtxoQuery.data ?? []}
          />
        </TabPanel>
        <TabPanel>
          <CoinjoinList
            isLoading={listCoinjoinRoomsQuery.isLoading}
            isError={listCoinjoinRoomsQuery.isError}
            deriv={deriv}
            data={listCoinjoinRoomsQuery.data ?? []}
          />
        </TabPanel>
      </TabPanels>
    </Tabs>
  );
};
