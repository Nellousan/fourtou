use std::{
    collections::HashMap,
    error::Error,
    ffi::{c_char, c_int, c_long},
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    os::fd::{AsFd, AsRawFd},
};

use nix::poll::{PollFd, PollFlags, PollTimeout};

#[allow(dead_code)]
#[link(name = "myteams")]
unsafe extern "C" {
    fn server_event_team_created(
        team_uuid: *const c_char,
        team_name: *const c_char,
        user_uuid: *const c_char,
    ) -> c_int;
    fn server_event_channel_created(
        team_uuid: *const c_char,
        channel_uuid: *const c_char,
        channel_name: *const c_char,
    ) -> c_int;
    fn server_event_thread_created(
        channel_uuid: *const c_char,
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        thread_title: *const c_char,
        thread_body: *const c_char,
    ) -> c_int;
    fn server_event_reply_created(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        reply_body: *const c_char,
    ) -> c_int;
    fn server_event_user_subscribed(team_uuid: *const c_char, user_uuid: *const c_char) -> c_int;
    fn server_event_user_unsubscribed(team_uuid: *const c_char, user_uuid: *const c_char) -> c_int;
    fn server_event_user_created(user_uuid: *const c_char, user_name: *const c_char) -> c_int;
    fn server_event_user_loaded(user_uuid: *const c_char, user_name: *const c_char) -> c_int;
    fn server_event_user_logged_in(user_uuid: *const c_char) -> c_int;
    fn server_event_user_logged_out(user_uuid: *const c_char) -> c_int;
    fn server_event_private_message_sended(
        sender_uuid: *const c_char,
        receiver_uuid: *const c_char,
        message_body: *const c_char,
    ) -> c_int;

    fn client_event_logged_in(user_uuid: *const c_char, user_name: *const c_char) -> c_int;
    fn client_event_logged_out(user_uuid: *const c_char, user_name: *const c_char) -> c_int;
    fn client_event_private_message_received(
        user_uuid: *const c_char,
        message_body: *const c_char,
    ) -> c_int;
    fn client_event_thread_reply_received(
        team_uuid: *const c_char,
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        reply_body: *const c_char,
    ) -> c_int;
    fn client_event_team_created(
        team_uuid: *const c_char,
        team_name: *const c_char,
        team_description: *const c_char,
    ) -> c_int;
    fn client_event_channel_created(
        channel_uuid: *const c_char,
        channel_name: *const c_char,
        channel_description: *const c_char,
    ) -> c_int;
    fn client_event_thread_created(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        thread_timestamp: c_long,
        thread_title: *const c_char,
        thread_body: *const c_char,
    ) -> c_int;
    fn client_print_users(
        user_uuid: *const c_char,
        user_name: *const c_char,
        user_status: c_int,
    ) -> c_int;
    fn client_print_teams(
        team_uuid: *const c_char,
        team_name: *const c_char,
        team_description: *const c_char,
    ) -> c_int;
    fn client_team_print_channels(
        channel_uuid: *const c_char,
        channel_name: *const c_char,
        channel_description: *const c_char,
    ) -> c_int;
    fn client_channel_print_threads(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        thread_timestamp: c_long,
        thread_title: *const c_char,
        thread_body: *const c_char,
    ) -> c_int;
    fn client_thread_print_replies(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        reply_timestamp: c_long,
        reply_body: *const c_char,
    ) -> c_int;
    fn client_private_message_print_messages(
        sender_uuid: *const c_char,
        message_timestamp: c_long,
        message_body: *const c_char,
    ) -> c_int;
    fn client_error_unknown_team(team_uuid: *const c_char) -> c_int;
    fn client_error_unknown_channel(channel_uuid: *const c_char) -> c_int;
    fn client_error_unknown_thread(thread_uuid: *const c_char) -> c_int;
    fn client_error_unknown_user(user_uuid: *const c_char) -> c_int;
    fn client_error_unauthorized() -> c_int;
    fn client_error_already_exist() -> c_int;
    fn client_print_user(
        user_uuid: *const c_char,
        user_name: *const c_char,
        user_status: c_int,
    ) -> c_int;
    fn client_print_team(
        team_uuid: *const c_char,
        team_name: *const c_char,
        team_description: *const c_char,
    ) -> c_int;
    fn client_print_channel(
        channel_uuid: *const c_char,
        channel_name: *const c_char,
        channel_description: *const c_char,
    ) -> c_int;
    fn client_print_thread(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        thread_timestamp: c_long,
        thread_title: *const c_char,
        thread_body: *const c_char,
    ) -> c_int;
    fn client_print_team_created(
        team_uuid: *const c_char,
        team_name: *const c_char,
        team_description: *const c_char,
    ) -> c_int;
    fn client_print_channel_created(
        channel_uuid: *const c_char,
        channel_name: *const c_char,
        channel_description: *const c_char,
    ) -> c_int;
    fn client_print_thread_created(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        thread_timestamp: c_long,
        thread_title: *const c_char,
        thread_body: *const c_char,
    ) -> c_int;
    fn client_print_reply_created(
        thread_uuid: *const c_char,
        user_uuid: *const c_char,
        reply_timestamp: c_long,
        reply_body: *const c_char,
    ) -> c_int;
    fn client_print_subscribed(user_uuid: *const c_char, team_uuid: *const c_char) -> c_int;
    fn client_print_unsubscribed(user_uuid: *const c_char, team_uuid: *const c_char) -> c_int;
}

pub struct Client {
    pub stream: TcpStream,
    pub socket: SocketAddr,
}

fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("0.0.0.0:8000")?;

    let mut clients: HashMap<i32, Client> = HashMap::new();

    loop {
        let mut pfds: Vec<PollFd> = vec![];

        pfds.push(PollFd::new(listener.as_fd(), PollFlags::POLLIN));

        for (_, client) in clients.iter() {
            pfds.push(PollFd::new(
                client.stream.as_fd(),
                PollFlags::POLLIN | PollFlags::POLLHUP,
            ));
        }

        nix::poll::poll(&mut pfds, PollTimeout::NONE)?;

        let mut new_clients = HashMap::new();
        let mut clients_to_read = vec![];
        for (i, fd) in pfds.iter().enumerate() {
            if let Some(revents) = fd.revents() {
                if revents.contains(PollFlags::POLLIN) {
                    if i == 0 {
                        println!("OUAIS J'AI");
                        let (new_client, sock) = listener.accept()?;
                        new_clients.insert(
                            new_client.as_fd().as_raw_fd(),
                            Client {
                                stream: new_client,
                                socket: sock,
                            },
                        );
                        continue;
                    }

                    clients_to_read.push(fd.as_fd().as_raw_fd());
                }
            }
        }

        for client_fd in clients_to_read.iter() {
            let mut buf = [0u8; 2048];
            let bytes_read = clients.get_mut(client_fd).unwrap().stream.read(&mut buf)?;
            if bytes_read == 0 {
                println!("Client {} disconnected", client_fd);
                let _ = clients.remove(client_fd);
                continue;
            }
            let message = String::from_utf8(buf.to_vec())?;
            println!("Client {}: {}", client_fd, message.trim());
        }

        clients.extend(new_clients);
    }
}
