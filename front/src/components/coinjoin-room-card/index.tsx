import {
  Box,
  Button,
  HStack,
  ListItem,
  Text,
  UnorderedList,
} from "@chakra-ui/react";
import Moment from "moment";

import { CoinJoinApi } from "@/apis";
import { RoomDto } from "@/dtos";
import { FC } from "react";

interface ICoinJoinRoomCard {
  key: any;
  data: RoomDto;
  deriv: string;
}

export const CoinJoinRoomCard: FC<ICoinJoinRoomCard> = (props) => {
  const { key, data } = props;
  return (
    <HStack
      key={key}
      color="white"
      textAlign="start"
      w="100%"
      bg="#3a3a3a"
      p="8px 16px"
      borderRadius="8px"
      minW="300px"
      justifyContent="space-between"
    >
      <Box>
        <Text fontSize="16px" fontWeight="700" w="100%">
          Room {data.id.slice(undefined, 8)}
        </Text>
        <UnorderedList w="100%">
          <ListItem>Amount: {data.base_amount} sats</ListItem>
          <ListItem>Number of peers: {data.no_peer}</ListItem>
          <ListItem>Status: {data.status}</ListItem>
          <ListItem>
            {"Due 1: " +
              Moment(data.created_at + data.due1).format("MMM DD, HH:mm")}
          </ListItem>
          <ListItem>
            {"Due 2: " +
              Moment(data.created_at + data.due1 + data.due2).format(
                "MMM DD, HH:mm",
              )}
          </ListItem>
        </UnorderedList>
      </Box>
      <Button
        onClick={() => {
          CoinJoinApi.signTxn(props.deriv, data.id);
        }}
      >
        Sign
      </Button>
    </HStack>
  );
};
