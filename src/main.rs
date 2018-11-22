extern crate libc;
use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

mod avcodec;
mod avformat;
mod avutil;
fn main() {
    unsafe {
        println!("hello world");
        let av_version_info_str = CStr::from_ptr(avutil::av_version_info()).to_string_lossy();
        println!("av_version_info : {}", av_version_info_str);

        println!("avformat_version_i : {}", avformat::avformat_version());

        let mut ps = ptr::null_mut();
        let mut pkt: avformat::AVPacket = mem::zeroed();
        avformat::av_init_packet(&mut pkt);
        let path = CString::new("D:\\workspace\\rust\\ffmpeg\\target\\debug\\test.mp4").unwrap();

        match avformat::avformat_open_input(
            &mut ps,
            path.as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
        ) {
            0 => {
                avformat::avformat_find_stream_info(ps, ptr::null_mut());
                let nb_streams = (*ps).nb_streams as usize;
                let streams = std::slice::from_raw_parts((*ps).streams, nb_streams);
                let url = CStr::from_ptr((*ps).url).to_string_lossy();
                println!("url : {}", url);
                println!("nb_streams : {}", nb_streams);
                let codec_id1 = (*(*streams[0]).codec).codec_id;
                let codec_id2 = (*(*streams[1]).codec).codec_id;
                let codec_name1 = avcodec::avcodec_get_name(codec_id1);
                let codec_name2 = avcodec::avcodec_get_name(codec_id2);
                let codec_type1 = avcodec::avcodec_get_type(codec_id1);
                let codec_type2 = avcodec::avcodec_get_type(codec_id2);

                println!("codec_type : {}", codec_type1);
                println!("codec : {}", CStr::from_ptr(codec_name1).to_string_lossy());
                println!("codec_type : {}", codec_type2);
                println!("codec : {}", CStr::from_ptr(codec_name2).to_string_lossy());


                loop{
                    match avformat::av_read_frame(ps, &mut pkt){
                        0 => {
                            println!("pkt size {}",pkt.size);
                            println!("pkt stream_index {}",pkt.stream_index);
                            avformat::av_packet_unref(&mut pkt);            
                            
                            
                        }
                        _ =>{
                            break;
                        }
                    }
                    
                }
                

                
                println!("succes");
            }
            e => {
                println!("fuck that {}", e);
            }
        }

        //let test = (*afc).filename;
        //let filename = CStr::from_ptr(test).to_string_lossy();
        //println!("avformat_version_i : {}", filename);
    }
}
