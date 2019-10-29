use futures::channel::mpsc::{ unbounded, UnboundedReceiver, UnboundedSender };
use futures::stream::StreamExt;
use std::future::Future;


pub struct Ring
{
	nodes: Option< Vec<Node> > ,
}


impl Ring
{
	// Create channels between all the nodes.
	//
	pub fn new( n: usize ) -> Self
	{
		assert!( n > 1 );

		let mut nodes = Vec::with_capacity( n );

		// The connection between the last and the first node
		//
		let (last_tx, last_rx) = unbounded();

		// The first node
		//
		let (tx, mut next_rx) = unbounded();

		nodes.push( Node { n, tx, rx: last_rx } );


		// All but first and last.
		// Note that 1..1 does not do anything, so it works if n==2.
		//
		for _ in 1..n-1
		{
			let (tx, rx) = unbounded();

			nodes.push( Node { n, tx, rx: next_rx } );

			next_rx = rx;
		}


		// The last node
		//
		nodes.push( Node { n, tx: last_tx, rx: next_rx } );


		Self
		{
			nodes: Some( nodes ),
		}
	}



	// Return a vec of futures that await the operation of each node.
	// You can either spawn them, join them, use futures unordered, ... to compare performance.
	//
	pub fn start_parallel( &mut self ) -> Vec<impl Future<Output = ()> >
	{
		self.nodes.take().unwrap().into_iter().map( move |mut node|
		{
			async move
			{
				node.run().await;
			}

		}).collect()
	}
}


// Each node will start by sending a 1 to the next node. When the counter has come back and
// been incremented by all nodes, the operation is complete.
//
pub struct Node
{
	n : usize                    ,
	tx: UnboundedSender  <usize> ,
	rx: UnboundedReceiver<usize> ,
}

impl Node
{
	async fn run( &mut self )
	{
		self.tx.unbounded_send( 1 ).expect( "send on unbouded" );


		while let Some(msg) = self.rx.next().await
		{
			// When our message comes back, it should be counted by everyone, so it should be n.
			// if the msg < n, it didn't originate here, send it on.
			//
			if msg == self.n
			{
				break
			}

			self.tx.unbounded_send( msg + 1 ).expect( "send on unbouded" );
		}
	}
}




#[ cfg( test ) ]
//
mod tests
{
	#[ allow( unused_imports ) ] // false positive
	//
	use super::*;

	#[test]
	//
	fn off_by_one_or_not()
	{
		let ring2 = Ring::new( 2 );
		assert_eq!( 2, ring2.nodes.unwrap().len() );

		let ring2 = Ring::new( 3 );
		assert_eq!( 3, ring2.nodes.unwrap().len() );
	}
}
