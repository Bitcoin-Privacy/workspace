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

interface CoinJoinRoomCardProps {
  key: any;
  data: RoomDto;
  deriv: string;
}

export function CoinJoinRoomCard(props: CoinJoinRoomCardProps) {
  const { key, data } = props;
  return (
    <HStack
      key={key}
      color="white"
      textAlign="start"
      maxW="300px"
      bg="#3a3a3a"
      p="8px 16px"
      borderRadius="8px"
      align="stretch"
      minW="300px"
      justifyContent="space-between"
    >
      <Box>
        <Text fontSize="16px" fontWeight="700" w="100%">
          Room {data.id.slice(undefined, 8)}
        </Text>
        <UnorderedList w="100%">
          <ListItem>Amount: {data.base_amount} sats</ListItem>
          <ListItem>Status: {data.status}</ListItem>
          <ListItem>
            Due 1: {Moment(data.created_at + data.due1).format("MMM DD, HH:mm")}
          </ListItem>
          <ListItem>
            Due 2:{" "}
            {Moment(data.created_at + data.due1 + data.due2).format(
              "MMM DD, HH:mm",
            )}
          </ListItem>
        </UnorderedList>
      </Box>
      <Button
        h="100%"
        onClick={() => {
          CoinJoinApi.signTxn(props.deriv, data.id);
        }}
      >
        Sign
      </Button>
    </HStack>
  );
}
