import React, { FC } from "react";
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

interface IProfilePanel {}

export const ProfilePannel: FC<IProfilePanel> = (props) => {
  const {
    states: {
      deriv,
      listUtxoQuery,
      listCoinjoinRoomsQuery,
      listTransferStatecoinsQuery,
      listStatecoinsQuery,
    },
  } = useProfilePage();
  return (
    <Tabs isFitted variant="unstyled" w="100%">
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
