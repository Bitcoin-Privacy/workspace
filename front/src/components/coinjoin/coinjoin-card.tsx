import { Box, HStack, ListItem, Text, UnorderedList } from "@chakra-ui/react";
import Moment from "moment";

import { RoomDto } from "@/dtos";
import { FC, useMemo } from "react";
import { CoinjoinStatus } from "./coinjoin-status";

interface ICoinjoinCard {
  key: any;
  data: RoomDto;
  deriv: string;
}

export const CoinjoinCard: FC<ICoinjoinCard> = (props) => {
  const { key, data, deriv } = props;

  const dues = useMemo(() => {
    return {
      endOfDue1: data.created_at + data.due1,
      endOfDue2: data.created_at + data.due1 + data.due2,
    };
  }, [data]);

  return (
    <HStack
      key={key}
      w="100%"
      bg="gray.900"
      borderRadius="8px"
      p="16px"
      justify="space-between"
      alignItems="center"
      minW="300px"
      color="white"
      textAlign="start"
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
            {"Due 1: " + Moment(dues.endOfDue1).format("MMM DD YYYY, HH:mm")}
          </ListItem>
          <ListItem>
            {"Due 2: " + Moment(dues.endOfDue2).format("MMM DD YYYY, HH:mm")}
          </ListItem>
        </UnorderedList>
      </Box>
      <CoinjoinStatus
        deriv={deriv}
        roomId={data.id}
        status={data.status}
        txid={data.txid}
        endOfDue1={data.created_at + data.due1}
        endOfDue2={data.created_at + data.due1 + data.due2}
      />
    </HStack>
  );
};
