import React, { useCallback, useEffect, useState } from "react";
import { Box, Text, Button, Tooltip, Flex } from "@chakra-ui/react";
import { useStatecoinDetail } from "@/hooks/atoms/use-statecoin-detail";
import {
  Copier,
  ExplorerLink,
  ExplorerLinkType,
  Layout,
  Loading,
} from "@/components";
import QRCodeGenerator from "@/components/qr-code-generator";
import { AppApi, StatechainApi } from "@/apis";
import { StatecoinDetailDto } from "@/dtos";
import { open } from "@tauri-apps/api/shell";
import { FaCheckCircle, FaClock } from "react-icons/fa";
import moment from "moment";
import { useRouter } from "next/router";
import { useNoti } from "@/hooks";
import { profilePath } from "@/utils";

export default function StatecoinDetail() {
  const { deriv, statechainId } = useStatecoinDetail();
  const [status, setStatus] = useState<boolean>(false);
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const [statecoin, setStatecoin] = useState<StatecoinDetailDto>();
  const router = useRouter();
  const getData = async (id: string) => {
    let res = await StatechainApi.getStatecoinDetailById(id);
    setStatecoin(res);
    let sta = await AppApi.getStatus(res.funding_txid);
    setStatus(sta);
  };

  useEffect(() => {
    if (statechainId) {
      getData(statechainId as string);
    }
  }, [statechainId]);

  const noti = useNoti();

  const onWithdrawBtnClick = useCallback(async () => {
    try {
      setIsLoading(true);
      let res = await StatechainApi.withdrawStatecoin(
        statecoin?.statechain_id as string,
        deriv,
      );
      console.log("Withdraw statecoin API response ", res);
      noti.success("Withdraw successfully");
      router.replace(profilePath(deriv, "?tab=STATECHAIN"));
    } catch (error: any) {
      console.log("Withdraw statecoin API error ", error);
      noti.success("Got an error!", error);
    } finally {
      setIsLoading(false);
    }
  }, [deriv, statecoin, noti, router]);

  if (!statecoin) {
    return (
      <Layout>
        <Loading content="Fetching info..." />
      </Layout>
    );
  }

  return (
    <Layout header title="Statecoin detail">
      <Flex
        mt="40px"
        h="full"
        borderRadius={"8"}
        justifyContent={"space-between"}
        px="36px"
        w={"full"}
      >
        <Flex
          direction={"column"}
          gap={"12px"}
          p={"10px 10px"}
          alignItems={"center"}
          justifyContent={"center"}
          flex="1"
        >
          <QRCodeGenerator text={statecoin?.funding_txid} size="200px" />
          <Tooltip label="View transaction onchain" placement="auto-end">
            <Button
              colorScheme="blue"
              variant="ghost"
              isTruncated
              maxW="80%"
              onClick={() => {
                open(
                  `https://blockstream.info/testnet/tx/${statecoin?.funding_txid}`,
                );
              }}
              p="4px"
              textAlign={{ base: "left", md: "left" }} // Center text on small screens
            >
              <Text isTruncated maxW="400px">
                {statecoin?.aggregated_address}
              </Text>
            </Button>
          </Tooltip>
          <Button
            w="80%"
            maxW="80%"
            colorScheme="red"
            onClick={onWithdrawBtnClick}
            isLoading={isLoading}
            isDisabled={isLoading}
          >
            Withdraw
          </Button>
          <Button w="80%" maxW="80%" colorScheme="teal" isDisabled={isLoading}>
            Broadcast backup transaction
          </Button>
          <Button
            w="80%"
            maxW="80%"
            variant="outline"
            colorScheme="whiteAlpha"
            isDisabled={isLoading}
            onClick={() => {
              router.back();
            }}
          >
            Go back
          </Button>
        </Flex>
        <Flex
          direction={"column"}
          gap={"24px"}
          alignItems="flex-start"
          p={6}
          flex="1"
          alignSelf={"center"}
          bg={"gray.800"}
          borderRadius={"16px"}
        >
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Statechain ID:
            </Text>
            <Text isTruncated maxW="400px">
              {statecoin.statechain_id}
            </Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Deposit transaction ID:
            </Text>
            <Copier content={statecoin.funding_txid} />
            <ExplorerLink
              id={statecoin.funding_txid}
              type={ExplorerLinkType.TRANSACTION}
            />
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Transaction Number (tx_n):
            </Text>
            <Text>{statecoin?.tx_n}</Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Aggregated Address:
            </Text>
            <Copier content={statecoin.aggregated_address} />
            <ExplorerLink
              id={statecoin.aggregated_address}
              type={ExplorerLinkType.ADDRESS}
            />
          </Box>

          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Amount:
            </Text>
            <Text>{statecoin?.amount} Sats</Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Expired date:
            </Text>
            <Text>
              {moment(statecoin.n_lock_time * 1000).format(
                "HH:mm MMM DD, YYYY",
              )}
            </Text>
          </Box>
          <Box>
            <Text fontSize={"larger"} fontWeight="bold">
              Created At:
            </Text>
            <Text>
              {moment(statecoin.created_at).format("HH:mm MMM DD, YYYY")}
            </Text>
          </Box>
          <Flex align="center" gap="10px">
            <Text fontSize={"larger"} fontWeight="bold">
              Confirm:
            </Text>
            {status ? (
              <FaCheckCircle color="#41c300" />
            ) : (
              <FaClock color="#fa8100" />
            )}
          </Flex>
        </Flex>
      </Flex>
    </Layout>
  );
}
